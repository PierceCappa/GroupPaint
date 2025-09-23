import { Component, signal } from '@angular/core';
import { PaintCanvas } from "./paint-canvas/paint-canvas";

@Component({
  selector: 'app-root',
  imports: [PaintCanvas],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  onUpdateColor(event: any) {
    // Handle color update event if needed
    console.log('Pixel updated:', event);
  }
}
