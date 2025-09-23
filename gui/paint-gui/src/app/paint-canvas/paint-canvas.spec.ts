import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PaintCanvas } from './paint-canvas';

describe('PaintCanvas', () => {
  let component: PaintCanvas;
  let fixture: ComponentFixture<PaintCanvas>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PaintCanvas]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PaintCanvas);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
