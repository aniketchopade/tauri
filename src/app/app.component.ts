import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'Tauri Angular Test App';

  greetingMessage = "";
  output: string = '';

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }

  async writeToTcp() {
    const requestData = { key: 'exampleKey', value: 'exampleValue' };
    try {
      const response = await invoke('invoke_tcp_socket', { requestData });
      this.output = JSON.stringify(response);
    } catch (error) {
      this.output = `Error: ${error}`;
    }
  }

  async getCpuUid() {
    try {
      const response = await invoke('get_cpu_uid');
      this.output = response as string;
    } catch (error) {
      this.output = `Error: ${error}`;
    }
  }

  async writeToRegistry() {
    const keyPath = 'Software\\MyApp';
    const valueName = 'MyValue';
    const valueData = 'SomeData';
    try {
      const response = await invoke('write_to_registry', { keyPath, valueName, valueData });
      this.output = response as string;
    } catch (error) {
      this.output = `Error: ${error}`;
    }
  }
}
