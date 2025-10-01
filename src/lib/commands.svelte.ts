import { invoke } from "@tauri-apps/api/core";

export const preventDefault = <T extends Event>(fn: (e: T) => void): ((e: T) => void) => {
	return (e: T) => {
		e.preventDefault();
		fn(e);
	};
};

export interface FileResult {
	filename: string;
	directory: string;
	last_modified: string | null;
}

export interface DatabaseConfig {
	host: string;
	port: number;
	user: string;
	password: string;
	database: string;
}

export interface Configuration {
	id: number;
	key: string;
	value: string;
	description: string | null;
	updated_at: string;
}

export class GlobalState {
	private _state = $state({ name: '', greet: '' });

	get greet() {
		return this._state.greet;
	}
	set greet(value: string) {
		this._state.greet = value;
	}
	get name() {
		return this._state.name;
	}
	set name(value: string) {
		this._state.name = value;
	}
	get nlen() {
		return this.name.length;
	}

	async submit() {
		this.greet = await invoke('greet', { name: this.name });
	}

	reset() {
		this.name = '';
		this.greet = '';
	}
}

export class SearchState {
	private _state = $state({ 
		searchTerms: '',
		results: [] as FileResult[],
		isLoading: false,
		error: '',
		useAndLogic: true, // true = AND (tutti i termini), false = OR (almeno uno)
		searchInFilenames: true // true = cerca nei filename, false = cerca nelle directory
	});

	get searchTerms() {
		return this._state.searchTerms;
	}
	set searchTerms(value: string) {
		this._state.searchTerms = value;
	}

	get results() {
		return this._state.results;
	}
	set results(value: FileResult[]) {
		this._state.results = value;
	}

	get isLoading() {
		return this._state.isLoading;
	}
	set isLoading(value: boolean) {
		this._state.isLoading = value;
	}

	get error() {
		return this._state.error;
	}
	set error(value: string) {
		this._state.error = value;
	}

	get useAndLogic() {
		return this._state.useAndLogic;
	}
	set useAndLogic(value: boolean) {
		this._state.useAndLogic = value;
	}

	get searchInFilenames() {
		return this._state.searchInFilenames;
	}
	set searchInFilenames(value: boolean) {
		this._state.searchInFilenames = value;
	}

	get hasSearchTerms() {
		return this.searchTerms.trim().length > 0;
	}

	get searchTermsArray() {
		return this.searchTerms
			.split(/[\s,]+/)
			.map(term => term.trim())
			.filter(term => term.length > 0);
	}

	async search(dbConfig: DatabaseConfig) {
		if (!this.hasSearchTerms) return;

		this.isLoading = true;
		this.error = '';
		this.results = [];

		try {
			const results = await invoke<FileResult[]>('search_files', { 
				searchTerms: this.searchTermsArray,
				dbConfig,
				useAndLogic: this.useAndLogic,
				searchInFilenames: this.searchInFilenames
			});
			this.results = results;
		} catch (err) {
			this.error = err as string;
		} finally {
			this.isLoading = false;
		}
	}

	reset() {
		this.searchTerms = '';
		this.results = [];
		this.error = '';
		this.isLoading = false;
	}
}

const STORAGE_KEY = 'nas_scanner_db_config';

const DEFAULT_CONFIG: DatabaseConfig = {
	host: 'localhost',
	port: 5432,
	user: 'postgres',
	password: '',
	database: 'nas_scanner'
};

export class SettingsState {
	private _state = $state({
		config: this.loadConfig(),
		isOpen: false,
		isSaving: false,
		connectionStatus: 'unknown' as 'connected' | 'disconnected' | 'unknown' | 'testing'
	});

	get config() {
		return this._state.config;
	}
	set config(value: DatabaseConfig) {
		this._state.config = value;
	}

	get isOpen() {
		return this._state.isOpen;
	}
	set isOpen(value: boolean) {
		this._state.isOpen = value;
	}

	get isSaving() {
		return this._state.isSaving;
	}
	set isSaving(value: boolean) {
		this._state.isSaving = value;
	}

	get connectionStatus() {
		return this._state.connectionStatus;
	}
	set connectionStatus(value: 'connected' | 'disconnected' | 'unknown' | 'testing') {
		this._state.connectionStatus = value;
	}

	private loadConfig(): DatabaseConfig {
		if (typeof window === 'undefined') return { ...DEFAULT_CONFIG };
		
		try {
			const stored = localStorage.getItem(STORAGE_KEY);
			if (stored) {
				return { ...DEFAULT_CONFIG, ...JSON.parse(stored) };
			}
		} catch (e) {
			console.error('Failed to load config:', e);
		}
		return { ...DEFAULT_CONFIG };
	}

	saveConfig() {
		if (typeof window === 'undefined') return;
		
		try {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(this.config));
		} catch (e) {
			console.error('Failed to save config:', e);
		}
	}

	open() {
		this.isOpen = true;
	}

	close() {
		this.isOpen = false;
	}

	async testConnection(): Promise<boolean> {
		this.connectionStatus = 'testing';
		try {
			const result = await invoke<boolean>('test_connection', { dbConfig: this.config });
			this.connectionStatus = result ? 'connected' : 'disconnected';
			return result;
		} catch (err) {
			console.error('Connection test failed:', err);
			this.connectionStatus = 'disconnected';
			return false;
		}
	}

	async save() {
		this.isSaving = true;
		await new Promise(resolve => setTimeout(resolve, 300)); // Simulate save delay
		this.saveConfig();
		this.isSaving = false;
		this.close();
		// Test connection after saving
		await this.testConnection();
	}

	reset() {
		this.config = { ...DEFAULT_CONFIG };
		this.saveConfig();
		this.connectionStatus = 'unknown';
	}
}

export class ConfigurationState {
	private _state = $state({
		configurations: [] as Configuration[],
		isLoading: false,
		isSaving: false,
		error: ''
	});

	get configurations() {
		return this._state.configurations;
	}
	set configurations(value: Configuration[]) {
		this._state.configurations = value;
	}

	get isLoading() {
		return this._state.isLoading;
	}
	set isLoading(value: boolean) {
		this._state.isLoading = value;
	}

	get isSaving() {
		return this._state.isSaving;
	}
	set isSaving(value: boolean) {
		this._state.isSaving = value;
	}

	get error() {
		return this._state.error;
	}
	set error(value: string) {
		this._state.error = value;
	}

	async loadConfigurations(dbConfig: DatabaseConfig) {
		this.isLoading = true;
		this.error = '';
		try {
			const configs = await invoke<Configuration[]>('get_configurations', { dbConfig });
			this.configurations = configs;
		} catch (err) {
			this.error = err as string;
		} finally {
			this.isLoading = false;
		}
	}

	async updateConfiguration(dbConfig: DatabaseConfig, key: string, value: string) {
		this.isSaving = true;
		this.error = '';
		try {
			await invoke('update_configuration', { dbConfig, key, value });
			// Ricarica le configurazioni
			await this.loadConfigurations(dbConfig);
		} catch (err) {
			this.error = err as string;
		} finally {
			this.isSaving = false;
		}
	}

	getConfigurationValue(key: string): string {
		const config = this.configurations.find(c => c.key === key);
		return config?.value || '';
	}

	getConfigurationDescription(key: string): string {
		const config = this.configurations.find(c => c.key === key);
		return config?.description || '';
	}
}
