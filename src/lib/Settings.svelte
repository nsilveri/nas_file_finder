<script lang="ts">
	import { SettingsState, preventDefault } from '$lib';

	interface Props {
		settings: SettingsState;
	}

	let { settings }: Props = $props();

	const onsubmit = preventDefault(() => settings.save());
	const onreset = () => settings.reset();
</script>

<!-- Settings Modal -->
<dialog class="modal" class:modal-open={settings.isOpen}>
	<div class="modal-box max-w-2xl">
		<form method="dialog">
			<button
				type="button"
				class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
				onclick={() => settings.close()}
			>
				✕
			</button>
		</form>

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

		<form {onsubmit} class="space-y-4">
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
				<span>Le impostazioni vengono salvate localmente nel browser.</span>
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

			<!-- Action Buttons -->
			<div class="modal-action">
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
				<button
					type="button"
					class="btn btn-ghost"
					onclick={() => settings.close()}
					disabled={settings.isSaving}
				>
					Annulla
				</button>
				<button type="submit" class="btn btn-primary" disabled={settings.isSaving}>
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
						Salva
					{/if}
				</button>
			</div>
		</form>
	</div>

	<!-- Modal backdrop -->
	<form method="dialog" class="modal-backdrop">
		<button type="button" onclick={() => settings.close()}>close</button>
	</form>
</dialog>
