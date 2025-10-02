import { languages, languageNames } from "../languages";

export class Selector {
    selector: HTMLSelectElement;

    constructor(selector: HTMLSelectElement) {
        this.selector = selector;
        this.populate(Object.keys(languageNames));

    }

    populate(items: string[] = []) {
        for (const name of items) {
            const option = document.createElement('option');
            option.value = name;
            option.textContent = name;
            this.selector.appendChild(option);
        }
    }

    onChange(callback: (language: string) => void) {
        this.selector.addEventListener('change', () => {
            const selectedLanguage = this.selector.value;
            if (selectedLanguage in languageNames) {
                callback(selectedLanguage);
            } else {
                console.error(`Language ${selectedLanguage} is not supported.`);
            }
        });
    }
}