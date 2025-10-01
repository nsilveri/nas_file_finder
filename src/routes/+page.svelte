<script lang="ts">
	import { SearchState, SettingsState, preventDefault } from '$lib';
	import type { FileResult } from '$lib';
	import Settings from '$lib/Settings.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	const searchState = new SearchState();
	const settingsState = new SettingsState();

	$inspect(searchState.results, searchState.isLoading);

	const onsubmit = preventDefault(() => searchState.hasSearchTerms && searchState.search(settingsState.config));
	const onreset = () => searchState.reset();

	// Test connection on mount
	onMount(() => {
		settingsState.testConnection();
	});

	// Get status badge color and text
	function getConnectionBadge() {
		switch (settingsState.connectionStatus) {
			case 'connected':
				return { color: 'badge-success', text: 'DB connesso', icon: '●' };
			case 'disconnected':
				return { color: 'badge-error', text: 'DB disconnesso', icon: '●' };
			case 'testing':
				return { color: 'badge-warning', text: 'Test...', icon: '◐' };
			default:
				return { color: 'badge-ghost', text: 'Sconosciuto', icon: '○' };
		}
	}

	// Group results by directory
	function groupByDirectory(results: FileResult[]): Map<string, FileResult[]> {
		const grouped = new Map<string, FileResult[]>();
		
		for (const result of results) {
			const dir = result.directory;
			if (!grouped.has(dir)) {
				grouped.set(dir, []);
			}
			grouped.get(dir)!.push(result);
		}
		
		// Sort files within each directory by filename
		for (const files of grouped.values()) {
			files.sort((a, b) => a.filename.localeCompare(b.filename));
		}
		
		return grouped;
	}

	// Reactive values using $derived
	let groupedResults = $derived(groupByDirectory(searchState.results));
	let totalFiles = $derived(searchState.results.length);
	let totalDirectories = $derived(groupedResults.size);

	// Open directory in file explorer
	async function openDirectory(directoryPath: string) {
		try {
			await invoke('open_directory', { directoryPath });
		} catch (error) {
			console.error('Failed to open directory:', error);
			alert(`Errore nell'apertura della directory: ${error}`);
		}
	}
</script>

<div class="h-screen bg-base-200 flex flex-col">
	<div class="container mx-auto max-w-6xl flex flex-col h-full">
		<!-- Header - Fixed -->
		<div class="flex-shrink-0 text-center pt-8 pb-4">
			<div class="flex justify-between items-center mb-4">
				<div class="flex-1 flex justify-start">
					<!-- Connection Status Badge -->
					<button
						type="button"
						class="badge {getConnectionBadge().color} gap-2 px-3 py-3 cursor-pointer hover:scale-105 transition-transform border-0"
						onclick={() => settingsState.testConnection()}
						title="Clicca per testare la connessione"
						aria-label="Testa connessione database"
					>
						<span class="text-lg" style="line-height: 0;">{getConnectionBadge().icon}</span>
						<span class="font-medium">{getConnectionBadge().text}</span>
					</button>
				</div>
				<h1 class="text-5xl font-bold flex-1">NAS File Finder</h1>
				<div class="flex-1 flex justify-end">
					<button
						class="btn btn-circle btn-ghost"
						onclick={() => settingsState.open()}
						title="Impostazioni"
						aria-label="Apri impostazioni database"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6"
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
					</button>
				</div>
			</div>
		</div>

		<!-- Search Form -->
		<div class="card bg-base-100 shadow-xl mb-6">
			<div class="card-body py-4">
				<form {onsubmit}>
					<div class="flex gap-2 items-center">
						<div class="form-control flex-1">
							<input
								id="search-input"
								bind:value={searchState.searchTerms}
								type="text"
								placeholder="Inserisci una o più parole da cercare (separate da spazi o virgole)..."
								class="input input-bordered w-full"
								disabled={searchState.isLoading}
								aria-label="Campo di ricerca file"
							/>
						</div>

						<!-- Filename/Directory Toggle -->
						<div class="form-control">
							<label class="label cursor-pointer gap-2 px-2">
								<span class="label-text font-medium" class:text-primary={searchState.searchInFilenames}>File</span>
								<input 
									type="checkbox" 
									bind:checked={searchState.searchInFilenames}
									class="toggle toggle-primary focus:outline-none focus:outline-0 focus:ring-0"
									disabled={searchState.isLoading}
									title={searchState.searchInFilenames ? 'Cerca nei nomi dei file' : 'Cerca nei percorsi delle directory'}
									aria-label="Seleziona dove cercare"
								/>
								<span class="label-text font-medium" class:text-primary={!searchState.searchInFilenames}>Dir</span>
							</label>
						</div>

						<!-- AND/OR Toggle -->
						<div class="form-control">
							<label class="label cursor-pointer gap-2 px-2">
								<span class="label-text font-medium" class:text-primary={searchState.useAndLogic}>AND</span>
								<input 
									type="checkbox" 
									bind:checked={searchState.useAndLogic}
									class="toggle toggle-primary focus:outline-none focus:outline-0 focus:ring-0"
									disabled={searchState.isLoading}
									title={searchState.useAndLogic ? 'Cerca file che contengono TUTTI i termini' : 'Cerca file che contengono ALMENO UN termine'}
									aria-label="Seleziona modalità di ricerca"
								/>
								<span class="label-text font-medium" class:text-primary={!searchState.useAndLogic}>OR</span>
							</label>
						</div>
						
						<button 
							type="submit"
							disabled={!searchState.hasSearchTerms || searchState.isLoading} 
							class="btn btn-primary"
							aria-label="Avvia ricerca"
						>
							{#if searchState.isLoading}
								<span class="loading loading-spinner loading-sm"></span>
								Ricerca...
							{:else}
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
								</svg>
								Cerca
							{/if}
						</button>

						{#if searchState.results.length > 0 || searchState.error}
							<button 
								type="button"
								onclick={onreset}
								class="btn btn-ghost"
								aria-label="Reset ricerca"
							>
								Reset
							</button>
						{/if}
					</div>
				</form>

				<!-- Error Message -->
				{#if searchState.error}
					<div class="alert alert-error mt-4">
						<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						<span>{searchState.error}</span>
					</div>
				{/if}
			</div>
		</div>

		<!-- Results - Scrollable -->
		<div class="flex-1 overflow-y-auto px-4 pb-4">
			{#if searchState.results.length > 0}
				<!-- Results Summary -->
				<div class="alert alert-info mb-6">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
					</svg>
					<span>
						Trovati <strong>{totalFiles}</strong> file in <strong>{totalDirectories}</strong> {totalDirectories === 1 ? 'directory' : 'directory'}
					</span>
				</div>

				<!-- Directory Cards -->
				<div class="space-y-4">
					{#each [...groupedResults.entries()] as [directory, files]}
						<div class="card bg-base-100 shadow-xl">
							<div class="card-body">
								<!-- Directory Header -->
								<div class="flex items-start gap-3 mb-4 pb-3 border-b border-base-300">
									<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-primary flex-shrink-0 mt-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
									</svg>
									<div class="flex-1 min-w-0">
										<h3 class="font-bold text-lg break-all">{directory}</h3>
										<p class="text-sm text-base-content/60 mt-1">
											{files.length} {files.length === 1 ? 'file' : 'file'}
										</p>
									</div>
									<button
										type="button"
										class="btn btn-sm btn-primary flex-shrink-0"
										onclick={() => openDirectory(directory)}
										title="Apri directory in Explorer"
									>
										<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
										</svg>
										Apri
									</button>
								</div>

								<!-- Files Grid -->
								<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
									{#each files as file}
										<div class="flex items-start gap-2 p-3 rounded-lg hover:bg-base-200 transition-colors border border-base-300">
											<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
											</svg>
											<div class="flex-1 min-w-0">
												<p class="font-semibold text-sm break-words mb-1" title={file.filename}>{file.filename}</p>
												{#if file.last_modified}
													<div class="flex items-center gap-1 text-xs text-base-content/60">
														<svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
															<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
														</svg>
														<span class="truncate">{file.last_modified}</span>
													</div>
												{/if}
											</div>
										</div>
									{/each}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{:else if !searchState.isLoading && searchState.searchTerms && !searchState.error}
				<div class="alert alert-warning">
					<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
					<span>Nessun file trovato per la ricerca specificata.</span>
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Settings Modal Component -->
<Settings settings={settingsState} />

<style>
	/* Remove focus ring from toggle switches */
	.toggle:focus,
	.toggle:focus-visible {
		outline: none !important;
		box-shadow: none !important;
		border-color: inherit !important;
	}
</style>
