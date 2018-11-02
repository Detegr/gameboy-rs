/* tslint:disable */
export function reset(arg0: Emulator): void;

export function run_until_redraw(arg0: Emulator): boolean;

export function display_buffer(arg0: Emulator): Uint8Array;

export function load_cartridge_data(arg0: Emulator, arg1: Uint8Array): void;

export function should_redraw_display(arg0: Emulator): boolean;

export function get_emulator(arg0: string): Emulator;

export class Emulator {
free(): void;

}
