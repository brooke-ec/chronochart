// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	interface Promise<T> {
		/**
		 * Displays a toast on rejection of the Promise
		 * @param title Toast title to display, defaults to "Error"
		 * @returns A Promise resolving to `void` if rejected
		 */
		display(title?: string): Promise<T>;
	}
}

export {};
