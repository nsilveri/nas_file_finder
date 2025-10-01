<script lang="ts">
	import { SettingsState, ConfigurationState, preventDefault } from '$lib';
	import { onMount } from 'svelte';

	interface Props {
		settings: SettingsState;
	}

	let { settings }: Props = $props();
	const configState = new ConfigurationState();
	let activeTab = $state<'database' | 'scanner'>('database');

	// Load configurations when modal opens and DB is connected
	$effect(() => {
		if (settings.isOpen && settings.connectionStatus === 'connected') {
			configState.loadConfigurations(settings.config);
		}
	});

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
				{#if settings.connectionStatus !== 'connected'}
					<div class="alert alert-warning">
						<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
						</svg>
						<span>Connetti al database per configurare le impostazioni del NAS Scanner</span>
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

					{#if configState.isLoading}
						<div class="flex justify-center py-4">
							<span class="loading loading-spinner loading-md"></span>
							<span class="ml-2">Caricamento configurazioni...</span>
						</div>
					{:else if configState.error}
						<div class="alert alert-error">
							<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
							</svg>
							<span>Errore nel caricamento: {configState.error}</span>
						</div>
					{:else}
						<div class="space-y-4">
							<!-- NAS Directory -->
							{#if configState.configurations.find(c => c.key === 'nas_directory')}
								<div class="form-control">
									<label class="label">
										<span class="label-text font-medium">Directory NAS</span>
									</label>
									<input
										type="text"
										value={configState.getConfigurationValue('nas_directory')}
										placeholder="\\192.168.1.160\service\CLIENTI"
										class="input input-bordered w-full"
										onchange={(e) => configState.updateConfiguration(settings.config, 'nas_directory', e.currentTarget.value)}
									/>
									<label class="label">
										<span class="label-text-alt">{configState.getConfigurationDescription('nas_directory')}</span>
									</label>
								</div>
							{/if}

							<!-- Scan Interval -->
							{#if configState.configurations.find(c => c.key === 'scan_interval')}
								<div class="form-control">
									<label class="label">
										<span class="label-text font-medium">Intervallo Scansione (secondi)</span>
									</label>
									<input
										type="number"
										value={configState.getConfigurationValue('scan_interval')}
										placeholder="10"
										min="1"
										class="input input-bordered w-full"
										onchange={(e) => configState.updateConfiguration(settings.config, 'scan_interval', e.currentTarget.value)}
									/>
									<label class="label">
										<span class="label-text-alt">{configState.getConfigurationDescription('scan_interval')}</span>
									</label>
								</div>
							{/if}

							<!-- Periodically Scan -->
							{#if configState.configurations.find(c => c.key === 'periodically_scan')}
								<div class="form-control">
									<label class="label cursor-pointer justify-start gap-4">
										<input
											type="checkbox"
											checked={configState.getConfigurationValue('periodically_scan') === 'true'}
											class="checkbox checkbox-primary"
											onchange={(e) => configState.updateConfiguration(settings.config, 'periodically_scan', e.currentTarget.checked ? 'true' : 'false')}
										/>
										<div>
											<span class="label-text font-medium">Scansione Periodica Automatica</span>
											<div class="label-text-alt">{configState.getConfigurationDescription('periodically_scan')}</div>
										</div>
									</label>
								</div>
							{/if}

							<!-- Scan Type -->
							{#if configState.configurations.find(c => c.key === 'scan_type')}
								<div class="form-control">
									<label class="label">
										<span class="label-text font-medium">Tipo di Scansione</span>
									</label>
									<select
										value={configState.getConfigurationValue('scan_type')}
										class="select select-bordered w-full"
										onchange={(e) => configState.updateConfiguration(settings.config, 'scan_type', e.currentTarget.value)}
									>
										<option value="full">Completa</option>
										<option value="range">Range Temporale</option>
									</select>
									<label class="label">
										<span class="label-text-alt">{configState.getConfigurationDescription('scan_type')}</span>
									</label>
								</div>
							{/if}

							<!-- Scan Days Back -->
							{#if configState.configurations.find(c => c.key === 'scan_days_back')}
								<div class="form-control">
									<label class="label">
										<span class="label-text font-medium">Giorni di Range (per scansione range)</span>
									</label>
									<input
										type="number"
										value={configState.getConfigurationValue('scan_days_back')}
										placeholder="30"
										min="1"
										class="input input-bordered w-full"
										onchange={(e) => configState.updateConfiguration(settings.config, 'scan_days_back', e.currentTarget.value)}
									/>
									<label class="label">
										<span class="label-text-alt">{configState.getConfigurationDescription('scan_days_back')}</span>
									</label>
								</div>
							{/if}

							<!-- Exclude Extensions -->
							{#if configState.configurations.find(c => c.key === 'exclude_extensions')}
								<div class="form-control">
									<label class="label">
										<span class="label-text font-medium">Estensioni Escluse (JSON Array)</span>
									</label>
									<textarea
										value={configState.getConfigurationValue('exclude_extensions')}
										placeholder='[".tmp", ".bak"]'
										class="textarea textarea-bordered h-20"
										onchange={(e) => configState.updateConfiguration(settings.config, 'exclude_extensions', e.currentTarget.value)}
									></textarea>
									<label class="label">
										<span class="label-text-alt">{configState.getConfigurationDescription('exclude_extensions')}</span>
									</label>
								</div>
							{/if}
						</div>
					{/if}

					{#if configState.isSaving}
						<div class="alert alert-info mt-4">
							<span class="loading loading-spinner loading-sm"></span>
							<span>Salvataggio configurazioni in corso...</span>
						</div>
					{/if}
				</div>
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
				{:else}
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
						<span class="text-sm">Le configurazioni vengono salvate automaticamente ne DB.</span>
					</div>
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
