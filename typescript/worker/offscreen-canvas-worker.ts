let offscreenCanvas: OffscreenCanvas | null = null;
let context: OffscreenCanvasRenderingContext2D | null = null;

addEventListener('message', async (event: MessageEvent<OffscreenCanvas | ImageBitmap>) => {
    if (event.data instanceof OffscreenCanvas) {
        offscreenCanvas = event.data;
        context = offscreenCanvas.getContext('2d')!;
        return;
    }
    if (!offscreenCanvas || !context) {
        return
    }
    const image = event.data;
    context.clearRect(0, 0, offscreenCanvas.width, offscreenCanvas.height);
    context.drawImage(image, 0, 0);
    image.close();
});