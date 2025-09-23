import { Component, ElementRef, ViewChild, Output, EventEmitter } from '@angular/core';

@Component({
  selector: 'app-paint-canvas',
  templateUrl: './paint-canvas.html',
  styleUrls: ['./paint-canvas.css']
})
export class PaintCanvas {
  @ViewChild('canvas', { static: true }) canvasRef!: ElementRef<HTMLCanvasElement>;
  @Output() UpdateColor = new EventEmitter<{x: number, y: number, color: [number, number, number]}>();

  width = 1000;
  height = 1000;
  color = '#000000';
  penWidth = 2;
  drawing = false;
  matrix: [number, number, number][][] = [];

  private ctx!: CanvasRenderingContext2D | null;

  ngOnInit() {
    this.ctx = this.canvasRef.nativeElement.getContext('2d');
    // Initialize matrix with white
    this.matrix = Array.from({ length: this.height }, () =>
      Array.from({ length: this.width }, () => [255, 255, 255])
    );
    this.redraw();
  }

  setColor(event: any) {
    this.color = event.target.value;
  }

  setPenWidth(event: any) {
    this.penWidth = +event.target.value;
  }

  onMouseDown(event: MouseEvent) {
    this.drawing = true;
  }

  onMouseUp() {
    this.drawing = false;
  }

  onMouseMove(event: MouseEvent) {
    if (this.drawing) {
      const x_y = this.mousePositionToMatrix(event.clientX, event.clientY);
      this.draw(x_y[0], x_y[1]);
    }
  }

  mousePositionToMatrix(mouseX: number, mouseY: number)
  {
    const rect = this.canvasRef.nativeElement.getBoundingClientRect();
    const x = Math.floor((mouseX - rect.left) * (this.width / rect.width));
    const y = Math.floor((mouseY - rect.top) * (this.height / rect.height));
    return [x, y];
  }

  //X and Y are the mouse x and y positions on the matrix.
  draw(x: number, y: number) {
    if (!this.ctx) return;
    this.ctx.fillStyle = this.color;
    this.ctx.beginPath();
    this.ctx.arc(x, y, this.penWidth / 2, 0, 2 * Math.PI);
    this.ctx.fill();

    // Update matrix and emit event
    const rgb = this.hexToRgb(this.color);
    for (let dx = -Math.floor(this.penWidth/2); dx <= Math.floor(this.penWidth/2); dx++) {
      for (let dy = -Math.floor(this.penWidth/2); dy <= Math.floor(this.penWidth/2); dy++) {
        const nx = x + dx, ny = y + dy;
        if (nx >= 0 && nx < this.width && ny >= 0 && ny < this.height) {
          this.matrix[ny][nx] = rgb;
          this.UpdateColor.emit({x: nx, y: ny, color: rgb});
        }
      }
    }
  }

  hexToRgb(hex: string): [number, number, number] {
    const v = hex.replace('#', '');
    return [
      parseInt(v.substring(0, 2), 16),
      parseInt(v.substring(2, 4), 16),
      parseInt(v.substring(4, 6), 16)
    ];
  }

  clearCanvas()
  {
    console.log("Button Pressed!");
  }

  redraw() {
    if (!this.ctx) return;
    const imgData = this.ctx.createImageData(this.width, this.height);
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        const idx = (y * this.width + x) * 4;
        const [r, g, b] = this.matrix[y][x];
        imgData.data[idx] = r;
        imgData.data[idx + 1] = g;
        imgData.data[idx + 2] = b;
        imgData.data[idx + 3] = 255;
      }
    }
    this.ctx.putImageData(imgData, 0, 0);
  }
}