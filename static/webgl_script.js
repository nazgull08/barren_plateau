import init, { run_app } from './pkg/vaporization.js';

async function run() {
    await init(); // Загружаем WASM-модуль
    run_app(); // Запускаем приложение, которое должно создать канвас

    const observer = new MutationObserver(mutations => {
        mutations.forEach(mutation => {
            mutation.addedNodes.forEach(node => {
                if (node.nodeName === 'CANVAS') {
                    resizeCanvas(node); // Настроить размер сразу после добавления
                    observer.disconnect(); // Отключаем наблюдатель после настройки
                    document.getElementById('loading').style.display = 'none'; // Скрываем полосу загрузки
                    
                    // Вешаем обработчик на canvas для активации Pointer Lock
                    node.addEventListener('click', () => {
                        if (document.pointerLockElement !== node) {
                            node.requestPointerLock();
                        }
                    });
                }
            });
        });
    });
    observer.observe(document.body, { childList: true });
}

function resizeCanvas(canvas) {
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

window.addEventListener('resize', () => {
    const canvas = document.querySelector('canvas');
    if (canvas) resizeCanvas(canvas);
});

run();
