<script lang="ts">
	import { SettingsState, ConfigurationState, preventDefault } from '$lib';
	import { onMount } from 'svelte';

	interface Props {
		settings: SettingsState;
	}

	interface HealthStatus {
		database: string;
		status: string;
		timestamp: string;
	}

	interface RecentFile {
		filename: string;
		directory: string;
		last_modified: string;
	}

	interface Statistics {
		total_files: number;
		total_folders: number;
		files_last_24h: number;
		folders_last_24h: number;
		last_scan: string;
		recent_files: RecentFile[];
		timestamp: string;
	}

	interface ScanStatus {
		last_scan: string;
		periodically_scan: string;
		scan_interval: string;
		scan_days_back: string;
		scan_type: string;
		timestamp: string;
	}

	let { settings }: Props = $props();
	const configState = new ConfigurationState();
	let activeTab = $state<'database' | 'scanner' | 'statistics'>('database');

	// Statistics state
	let healthStatus = $state<HealthStatus | null>(null);
	let statistics = $state<Statistics | null>(null);
	let scanStatus = $state<ScanStatus | null>(null);
	let isLoadingStats = $state(false);
	let statsError = $state<string | null>(null);

	// Scanner configurations from API
	interface ApiConfiguration {
		key: string;
		value: string;
		description: string;
		updated_at: string | null;
	}

	let apiConfigurations = $state<ApiConfiguration[]>([]);
	let isLoadingApiConfigs = $state(false);
	let apiConfigError = $state<string | null>(null);
	let isSavingApiConfig = $state(false);

	// Local backend server IP (saved in localStorage only)
	let backendServerIP = $state<string>('');

	// Load backend IP from localStorage on mount
	$effect(() => {
		if (settings.isOpen) {
			const savedIP = localStorage.getItem('backend_server_ip');
			if (savedIP) {
				backendServerIP = savedIP;
			} else {
				// Default to database host if not set
				backendServerIP = settings.config.host || 'localhost';
			}
		}
	});

	// Get API base URL from localStorage backend_server IP
	function getApiBaseUrl(): string {
		const host = backendServerIP || settings.config.host || 'localhost';
		return `http://${host}:5000`;
	}

	function saveBackendServerIP(ip: string) {
		backendServerIP = ip;
		localStorage.setItem('backend_server_ip', ip);
	}

	// Load configurations from API when scanner tab is opened
	$effect(() => {
		if (settings.isOpen && activeTab === 'scanner') {
			loadApiConfigurations();
		}
	});

	// Load statistics when statistics tab is opened
	$effect(() => {
		if (settings.isOpen && activeTab === 'statistics') {
			loadStatistics();
		}
	});

	async function loadApiConfigurations() {
		isLoadingApiConfigs = true;
		apiConfigError = null;

		try {
			const apiBaseUrl = getApiBaseUrl();
			console.log('Fetching configurations from:', `${apiBaseUrl}/api/configurations`);
			
			const response = await fetch(`${apiBaseUrl}/api/configurations`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
				// Add timeout
				signal: AbortSignal.timeout(5000)
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}

			const data = await response.json();
			console.log('Configurations loaded:', data);
			apiConfigurations = data.configurations || [];
		} catch (error) {
			if (error instanceof Error) {
				if (error.name === 'TimeoutError') {
					apiConfigError = `Timeout: Il backend non risponde su ${getApiBaseUrl()}`;
				} else if (error.message.includes('Failed to fetch')) {
					apiConfigError = `Impossibile raggiungere il backend su ${getApiBaseUrl()}. Verifica che sia avviato e accessibile.`;
				} else {
					apiConfigError = error.message;
				}
			} else {
				apiConfigError = 'Errore sconosciuto';
			}
			console.error('Error loading API configurations:', error);
		} finally {
			isLoadingApiConfigs = false;
		}
	}

	async function updateApiConfiguration(key: string, value: string) {
		isSavingApiConfig = true;

		try {
			const apiBaseUrl = getApiBaseUrl();
			console.log(`Updating configuration ${key}:`, value);
			
			const response = await fetch(`${apiBaseUrl}/api/configurations/${key}`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({ value }),
				signal: AbortSignal.timeout(5000)
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({ error: response.statusText }));
				throw new Error(errorData.error || `HTTP ${response.status}`);
			}

			// Update local state
			const configIndex = apiConfigurations.findIndex(c => c.key === key);
			if (configIndex !== -1) {
				apiConfigurations[configIndex].value = value;
				apiConfigurations[configIndex].updated_at = new Date().toISOString();
			}
			
			console.log(`Configuration ${key} updated successfully`);
		} catch (error) {
			console.error('Error updating API configuration:', error);
			const errorMessage = error instanceof Error ? error.message : 'Errore sconosciuto';
			alert('Errore nel salvataggio: ' + errorMessage);
		} finally {
			isSavingApiConfig = false;
		}
	}

	function getApiConfigValue(key: string): string {
		const config = apiConfigurations.find(c => c.key === key);
		return config?.value || '';
	}

	function getApiConfigDescription(key: string): string {
		const config = apiConfigurations.find(c => c.key === key);
		return config?.description || '';
	}

	async function loadStatistics() {
		isLoadingStats = true;
		statsError = null;
		
		try {
			const apiBaseUrl = getApiBaseUrl();
			// Fetch all three endpoints in parallel
			const [healthRes, statsRes, scanRes] = await Promise.all([
				fetch(`${apiBaseUrl}/api/health`),
				fetch(`${apiBaseUrl}/api/statistics`),
				fetch(`${apiBaseUrl}/api/scan/status`)
			]);

			if (!healthRes.ok || !statsRes.ok || !scanRes.ok) {
				throw new Error('Errore nel caricamento delle statistiche');
			}

			healthStatus = await healthRes.json();
			statistics = await statsRes.json();
			scanStatus = await scanRes.json();
		} catch (error) {
			statsError = error instanceof Error ? error.message : 'Errore sconosciuto';
			console.error('Error loading statistics:', error);
		} finally {
			isLoadingStats = false;
		}
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleString('it-IT', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}

	function formatNumber(num: number): string {
		return num.toLocaleString('it-IT');
	}

	const onsubmit = preventDefault(() => settings.save());
	const onreset = () => settings.reset();
</script>

<!-- Settings Modal -->
<dialog class="modal" class:modal-open={settings.isOpen}>
	<div class="modal-box max-w-2xl flex flex-col h-[90vh]">
		<form method="dialog">
			<button
				type="button"
				class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2 z-10"
				onclick={() => settings.close()}
			>
				✕
			</button>
		</form>

		<!-- Fixed Header -->
		<div class="flex-shrink-0">
			<h3 class="font-bold text-2xl mb-6">
				<div class="flex items-center gap-2">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-7 w-7"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
						/>
					</svg>
					Impostazioni
				</div>
			</h3>

			<div class="divider"></div>

			<!-- Fixed Tabs -->
			<div class="tabs tabs-boxed mb-4">
				<button 
					type="button"
					class="tab"
					class:tab-active={activeTab === 'database'}
					onclick={() => activeTab = 'database'}
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
					</svg>
					Database
				</button>
				<button 
					type="button"
					class="tab"
					class:tab-active={activeTab === 'scanner'}
					onclick={() => activeTab = 'scanner'}
					disabled={settings.connectionStatus !== 'connected'}
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z" />
					</svg>
					NAS Scanner
				</button>
				<button 
					type="button"
					class="tab"
					class:tab-active={activeTab === 'statistics'}
					onclick={() => activeTab = 'statistics'}
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
					</svg>
					Statistiche
				</button>
			</div>
		</div>

		<!-- Scrollable Content -->
		<div class="flex-1 overflow-y-auto pr-2">
			<form {onsubmit} class="space-y-4" data-settings-form>
			<!-- Database Tab Content -->
			{#if activeTab === 'database'}
				<!-- PostgreSQL Configuration Section -->
				<div class="bg-base-200 p-4 rounded-lg">
				<h4 class="font-semibold text-lg mb-4 flex items-center gap-2">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
						/>
					</svg>
					Configurazione Database PostgreSQL
				</h4>

				<div class="space-y-3">
					<!-- Host -->
					<div class="form-control">
						<label class="label" for="db-host">
							<span class="label-text font-medium">Host</span>
						</label>
						<input
							id="db-host"
							bind:value={settings.config.host}
							type="text"
							placeholder="localhost"
							class="input input-bordered w-full"
							required
						/>
						<label class="label">
							<span class="label-text-alt">Indirizzo del server PostgreSQL</span>
						</label>
					</div>

					<!-- Port -->
					<div class="form-control">
						<label class="label" for="db-port">
							<span class="label-text font-medium">Porta</span>
						</label>
						<input
							id="db-port"
							bind:value={settings.config.port}
							type="number"
							placeholder="5432"
							class="input input-bordered w-full"
							min="1"
							max="65535"
							required
						/>
						<label class="label">
							<span class="label-text-alt">Porta del server PostgreSQL (default: 5432)</span>
						</label>
					</div>

					<!-- Username -->
					<div class="form-control">
						<label class="label" for="db-user">
							<span class="label-text font-medium">Username</span>
						</label>
						<input
							id="db-user"
							bind:value={settings.config.user}
							type="text"
							placeholder="postgres"
							class="input input-bordered w-full"
							required
						/>
						<label class="label">
							<span class="label-text-alt">Nome utente del database</span>
						</label>
					</div>

					<!-- Password -->
					<div class="form-control">
						<label class="label" for="db-password">
							<span class="label-text font-medium">Password</span>
						</label>
						<input
							id="db-password"
							bind:value={settings.config.password}
							type="password"
							placeholder="••••••••"
							class="input input-bordered w-full"
						/>
						<label class="label">
							<span class="label-text-alt">Password del database (opzionale)</span>
						</label>
					</div>

					<!-- Database Name -->
					<div class="form-control">
						<label class="label" for="db-name">
							<span class="label-text font-medium">Nome Database</span>
						</label>
						<input
							id="db-name"
							bind:value={settings.config.database}
							type="text"
							placeholder="nas_scanner"
							class="input input-bordered w-full"
							required
						/>
						<label class="label">
							<span class="label-text-alt">Nome del database da utilizzare</span>
						</label>
					</div>
				</div>
			</div>

				<!-- Info Alert -->
				<div class="alert alert-info">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						class="stroke-current shrink-0 w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						></path>
					</svg>
					<span>Le impostazioni relative al DB vengono salvate nell'applicazione.</span>
				</div>

				<!-- Test Connection Button -->
				<div class="flex justify-center">
					<button
						type="button"
						class="btn btn-outline btn-info"
						onclick={() => settings.testConnection()}
						disabled={settings.connectionStatus === 'testing'}
					>
						{#if settings.connectionStatus === 'testing'}
							<span class="loading loading-spinner loading-sm"></span>
							Test in corso...
						{:else if settings.connectionStatus === 'connected'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5 text-success"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							Connessione OK
						{:else if settings.connectionStatus === 'disconnected'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5 text-error"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							Connessione Fallita
						{:else}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13 10V3L4 14h7v7l9-11h-7z"
								/>
							</svg>
							Testa Connessione
						{/if}
					</button>
				</div>
			{/if}

			<!-- Scanner Tab Content -->
			{#if activeTab === 'scanner'}
				<!-- Backend Configuration Section (always visible) -->
				<div class="bg-base-200 p-4 rounded-lg mb-4">
					<h4 class="font-semibold text-lg mb-4 flex items-center gap-2">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.121 17.804A13.937 13.937 0 0112 16c2.5 0 4.847.655 6.879 1.804M15 10a3 3 0 11-6 0 3 3 0 016 0zm6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						Configurazione Backend
					</h4>

					<div class="space-y-4">
						<!-- Backend Server IP (localStorage only) -->
						<div class="form-control">
							<label class="label">
								<span class="label-text font-medium">Indirizzo IP Backend</span>
							</label>
							<input
								type="text"
								value={backendServerIP}
								placeholder="192.168.1.160"
								class="input input-bordered w-full"
								onchange={(e) => saveBackendServerIP(e.currentTarget.value)}
							/>
							<label class="label">
								<span class="label-text-alt">Indirizzo IP del server backend (porta 5000). Salvato nell'applicazione.</span>
							</label>
						</div>

						<!-- Test Backend Connection -->
						<div class="flex justify-center">
							<button
								type="button"
								class="btn btn-outline btn-info btn-sm"
								onclick={() => loadApiConfigurations()}
								disabled={isLoadingApiConfigs}
							>
								{#if isLoadingApiConfigs}
									<span class="loading loading-spinner loading-xs"></span>
									Test in corso...
								{:else}
									<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
									</svg>
									Testa Connessione Backend
								{/if}
							</button>
						</div>
					</div>
				</div>

				<!-- API Configurations (loaded from backend) -->
				{#if isLoadingApiConfigs}
					<div class="flex flex-col items-center justify-center py-8">
						<span class="loading loading-spinner loading-lg"></span>
						<span class="mt-4">Caricamento configurazioni...</span>
					</div>
				{:else if apiConfigError}
					<div class="alert alert-error">
						<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						<div>
							<div class="font-bold">Errore di connessione</div>
							<div class="text-sm">{apiConfigError}</div>
						</div>
					</div>
					<div class="mt-4">
						<div class="text-sm mb-2">
							<strong>Suggerimenti:</strong>
							<ul class="list-disc list-inside mt-2 space-y-1">
								<li>Verifica che il backend Python sia avviato sulla porta 5000</li>
								<li>Controlla che l'IP configurato ({getApiBaseUrl()}) sia corretto</li>
								<li>Verifica che non ci siano firewall che bloccano la connessione</li>
								<li>Assicurati che il backend abbia CORS abilitato per il frontend</li>
							</ul>
						</div>
					</div>
					<div class="mt-4 text-center">
						<button type="button" class="btn btn-primary" onclick={() => loadApiConfigurations()}>
							<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
							</svg>
							Riprova Connessione
						</button>
					</div>
				{:else}
					<!-- NAS Configuration Section -->
				<div class="bg-base-200 p-4 rounded-lg">
					<h4 class="font-semibold text-lg mb-4 flex items-center gap-2">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z" />
						</svg>
						Configurazioni NAS Scanner
					</h4>

					<div class="space-y-4">
						<!-- Remove Backend Server from API configs since it's now handled separately -->
						
						<!-- NAS Directory -->
						{#if apiConfigurations.find(c => c.key === 'nas_directory')}
							<div class="form-control">
								<label class="label">
									<span class="label-text font-medium">Directory NAS</span>
								</label>
								<input
									type="text"
									value={getApiConfigValue('nas_directory')}
									placeholder="\\192.168.1.160\service\CLIENTI"
									class="input input-bordered w-full"
									onchange={(e) => updateApiConfiguration('nas_directory', e.currentTarget.value)}
								/>
								<label class="label">
									<span class="label-text-alt">{getApiConfigDescription('nas_directory')}</span>
								</label>
							</div>
						{/if}

						<!-- Scan Interval -->
						{#if apiConfigurations.find(c => c.key === 'scan_interval')}
							<div class="form-control">
								<label class="label">
									<span class="label-text font-medium">Intervallo Scansione (secondi)</span>
								</label>
								<input
									type="number"
									value={getApiConfigValue('scan_interval')}
									placeholder="3600"
									min="1"
									class="input input-bordered w-full"
									onchange={(e) => updateApiConfiguration('scan_interval', e.currentTarget.value)}
								/>
								<label class="label">
									<span class="label-text-alt">{getApiConfigDescription('scan_interval')}</span>
								</label>
							</div>
						{/if}

						<!-- Periodically Scan -->
						{#if apiConfigurations.find(c => c.key === 'periodically_scan')}
							<div class="form-control">
								<label class="label cursor-pointer justify-start gap-4">
									<input
										type="checkbox"
										checked={getApiConfigValue('periodically_scan') === 'true'}
										class="checkbox checkbox-primary"
										onchange={(e) => updateApiConfiguration('periodically_scan', e.currentTarget.checked ? 'true' : 'false')}
									/>
									<div>
										<span class="label-text font-medium">Scansione Periodica Automatica</span>
										<div class="label-text-alt">{getApiConfigDescription('periodically_scan')}</div>
									</div>
								</label>
							</div>
						{/if}

						<!-- Scan Type -->
						{#if apiConfigurations.find(c => c.key === 'scan_type')}
							<div class="form-control">
								<label class="label">
									<span class="label-text font-medium">Tipo di Scansione</span>
								</label>
								<select
									value={getApiConfigValue('scan_type')}
									class="select select-bordered w-full"
									onchange={(e) => updateApiConfiguration('scan_type', e.currentTarget.value)}
								>
									<option value="full">Completa</option>
									<option value="range">Range Temporale</option>
								</select>
								<label class="label">
									<span class="label-text-alt">{getApiConfigDescription('scan_type')}</span>
								</label>
							</div>
						{/if}

						<!-- Scan Days Back -->
						{#if apiConfigurations.find(c => c.key === 'scan_days_back')}
							<div class="form-control">
								<label class="label">
									<span class="label-text font-medium">Giorni di Range (per scansione range)</span>
								</label>
								<input
									type="number"
									value={getApiConfigValue('scan_days_back')}
									placeholder="30"
									min="1"
									class="input input-bordered w-full"
									onchange={(e) => updateApiConfiguration('scan_days_back', e.currentTarget.value)}
								/>
								<label class="label">
									<span class="label-text-alt">{getApiConfigDescription('scan_days_back')}</span>
								</label>
							</div>
						{/if}

						<!-- Exclude Extensions -->
						{#if apiConfigurations.find(c => c.key === 'exclude_extensions')}
							<div class="form-control">
								<label class="label">
									<span class="label-text font-medium">Estensioni Escluse (JSON Array)</span>
								</label>
								<textarea
									value={getApiConfigValue('exclude_extensions')}
									placeholder='[".tmp", ".bak"]'
									class="textarea textarea-bordered h-20"
									onchange={(e) => updateApiConfiguration('exclude_extensions', e.currentTarget.value)}
								></textarea>
								<label class="label">
									<span class="label-text-alt">{getApiConfigDescription('exclude_extensions')}</span>
								</label>
							</div>
						{/if}
					</div>

					{#if isSavingApiConfig}
						<div class="alert alert-info mt-4">
							<span class="loading loading-spinner loading-sm"></span>
							<span>Salvataggio configurazione in corso...</span>
						</div>
					{/if}
				</div>
				{/if}
			{/if}

			<!-- Statistics Tab Content -->
			{#if activeTab === 'statistics'}
				{#if isLoadingStats}
					<div class="flex flex-col items-center justify-center py-8">
						<span class="loading loading-spinner loading-lg"></span>
						<span class="mt-4">Caricamento statistiche...</span>
					</div>
				{:else if statsError}
					<div class="alert alert-error">
						<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						<span>{statsError}</span>
					</div>
				{:else}
					<!-- Health Status -->
					{#if healthStatus}
						<div class="bg-base-200 p-4 rounded-lg mb-4">
							<h4 class="font-semibold text-lg mb-3 flex items-center gap-2">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
								</svg>
								Stato Sistema
							</h4>
							<div class="grid grid-cols-2 gap-3">
								<div>
									<span class="text-sm opacity-70">Database:</span>
									<div class="font-semibold badge badge-success gap-2">
										{healthStatus.database}
									</div>
								</div>
								<div>
									<span class="text-sm opacity-70">Status:</span>
									<div class="font-semibold badge badge-success gap-2">
										{healthStatus.status}
									</div>
								</div>
							</div>
						</div>
					{/if}

					<!-- Statistics Overview -->
					{#if statistics}
						<div class="bg-base-200 p-4 rounded-lg mb-4">
							<h4 class="font-semibold text-lg mb-3 flex items-center gap-2">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
								</svg>
								Statistiche Database
							</h4>
							<div class="grid grid-cols-2 gap-4">
								<div class="stat bg-base-100 rounded-lg p-3">
									<div class="stat-title text-xs">Files Totali</div>
									<div class="stat-value text-2xl text-primary">{formatNumber(statistics.total_files)}</div>
								</div>
								<div class="stat bg-base-100 rounded-lg p-3">
									<div class="stat-title text-xs">Cartelle Totali</div>
									<div class="stat-value text-2xl text-secondary">{formatNumber(statistics.total_folders)}</div>
								</div>
								<div class="stat bg-base-100 rounded-lg p-3">
									<div class="stat-title text-xs">Files (ultime 24h)</div>
									<div class="stat-value text-2xl text-accent">{formatNumber(statistics.files_last_24h)}</div>
								</div>
								<div class="stat bg-base-100 rounded-lg p-3">
									<div class="stat-title text-xs">Cartelle (ultime 24h)</div>
									<div class="stat-value text-2xl text-accent">{formatNumber(statistics.folders_last_24h)}</div>
								</div>
							</div>
							<div class="mt-3">
								<span class="text-sm opacity-70">Ultima scansione:</span>
								<div class="font-mono text-sm">{formatDate(statistics.last_scan)}</div>
							</div>
						</div>
					{/if}

					<!-- Scan Status -->
					{#if scanStatus}
						<div class="bg-base-200 p-4 rounded-lg mb-4">
							<h4 class="font-semibold text-lg mb-3 flex items-center gap-2">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
								</svg>
								Stato Scansione
							</h4>
							<div class="space-y-2">
								<div class="flex justify-between">
									<span class="text-sm opacity-70">Scansione Periodica:</span>
									<span class="badge badge-{scanStatus.periodically_scan === 'true' ? 'success' : 'error'}">
										{scanStatus.periodically_scan === 'true' ? 'Attiva' : 'Disattivata'}
									</span>
								</div>
								<div class="flex justify-between">
									<span class="text-sm opacity-70">Tipo Scansione:</span>
									<span class="badge badge-info">{scanStatus.scan_type === 'full' ? 'Completa' : 'Range Temporale'}</span>
								</div>
								<div class="flex justify-between">
									<span class="text-sm opacity-70">Intervallo:</span>
									<span class="font-semibold">{scanStatus.scan_interval} secondi</span>
								</div>
								<div class="flex justify-between">
									<span class="text-sm opacity-70">Giorni Range:</span>
									<span class="font-semibold">{scanStatus.scan_days_back} giorni</span>
								</div>
								<div class="divider my-2"></div>
								<div>
									<span class="text-sm opacity-70">Ultima scansione:</span>
									<div class="font-mono text-sm">{formatDate(scanStatus.last_scan)}</div>
								</div>
							</div>
						</div>
					{/if}

					<!-- Recent Files -->
					{#if statistics && statistics.recent_files && statistics.recent_files.length > 0}
						<div class="bg-base-200 p-4 rounded-lg">
							<h4 class="font-semibold text-lg mb-3 flex items-center gap-2">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
								</svg>
								File Recenti (ultimi 10)
							</h4>
							<div class="overflow-x-auto">
								<table class="table table-xs table-pin-rows">
									<thead>
										<tr>
											<th>Nome File</th>
											<th>Directory</th>
											<th>Ultima Modifica</th>
										</tr>
									</thead>
									<tbody>
										{#each statistics.recent_files as file}
											<tr class="hover">
												<td class="font-mono text-xs">{file.filename}</td>
												<td class="text-xs opacity-70 max-w-xs truncate" title={file.directory}>{file.directory}</td>
												<td class="text-xs">{formatDate(file.last_modified)}</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>
					{/if}
				{/if}
			{/if}
		</form>
		</div>

		<!-- Fixed Footer with Action Buttons -->
		<div class="flex-shrink-0 mt-4 pt-4 border-t border-base-300">
			<div class="flex justify-between gap-2">
				<!-- Show Reset and Save only for Database tab -->
				{#if activeTab === 'database'}
					<button type="button" class="btn btn-ghost" onclick={onreset}>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
							/>
						</svg>
						Ripristina Default
					</button>
					<div class="flex gap-2">
						<button
							type="button"
							class="btn btn-ghost"
							onclick={() => settings.close()}
							disabled={settings.isSaving}
						>
							Annulla
						</button>
						<button 
							type="button" 
							class="btn btn-primary" 
							disabled={settings.isSaving}
							onclick={() => {
								const form = document.querySelector('form[data-settings-form]') as HTMLFormElement;
								if (form) form.requestSubmit();
							}}
						>
							{#if settings.isSaving}
								<span class="loading loading-spinner loading-sm"></span>
								Salvataggio...
							{:else}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="h-5 w-5"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 13l4 4L19 7"
									/>
								</svg>
								Salva Configurazione DB
							{/if}
						</button>
					</div>
				{:else if activeTab === 'scanner'}
					<!-- Info Alert and Close button for Scanner tab -->
					<div class="alert alert-info py-2 px-3 flex-1 mr-2">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							class="stroke-current shrink-0 w-5 h-5"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							></path>
						</svg>
						<span class="text-sm">Le configurazioni vengono salvate automaticamente nel backend SQLite.</span>
					</div>
					<button
						type="button"
						class="btn btn-ghost"
						onclick={() => settings.close()}
					>
						Chiudi
					</button>
				{:else if activeTab === 'statistics'}
					<!-- Refresh and Close buttons for Statistics tab -->
					<button
						type="button"
						class="btn btn-outline btn-info"
						onclick={() => loadStatistics()}
						disabled={isLoadingStats}
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
						Aggiorna
					</button>
					<button
						type="button"
						class="btn btn-ghost"
						onclick={() => settings.close()}
					>
						Chiudi
					</button>
				{/if}
			</div>
		</div>
	</div>

	<!-- Modal backdrop -->
	<form method="dialog" class="modal-backdrop">
		<button type="button" onclick={() => settings.close()}>close</button>
	</form>
</dialog>
