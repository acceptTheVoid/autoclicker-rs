use winput::{message_loop::{self, Event}, Button, Vk};
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration, process};

static TOGGLE_BUTTON: Vk = Vk::F6;
static EXIT_BUTTON: Vk = Vk::F12;

fn main() {
    // В винде каждое нажатие клавиши/мыши это событие
    // Так что я буду сидеть и слуштаь все события в этой богом забытой системе
    let events = message_loop::start().unwrap();

    // По сути простая проверка на два состояния включен кликер или нет
    // Занимает целую кучу куда благодаря синхронизации между потоками
    // Выглядит тупо но по сути весь низкоуровневый многопоток так выглядит
    let active_change = Arc::new(AtomicBool::new(false));
    let active_check = active_change.clone();

    // Поток для самого кликера 
    let clicker = thread::spawn(move || {
        // Бесконечный цикл который проверяет активен ли кликер
        // Если активен то кликер соответственно кликает
        // Дикие программы удивительны не правда ли
        loop {
            if active_check.load(Ordering::Relaxed) {
                winput::send(Button::Left);
                // Если тут не дать потоку отдохнуть то все ломается так что....
                thread::sleep(Duration::from_millis(1));
            }
        }
    });

    // Поток для включения/выключения кликера
    let toggle_clicker = thread::spawn(move || {
        loop {
            match events.next_event() {
                Event::Keyboard { vk, .. } => {
                    match vk {
                        // Если клавиша равна клавише включения/выключения кликера то мы
                        // Выполняем соответствующее действие
                        k if k == TOGGLE_BUTTON  => {
                            let to_store = !active_change.load(Ordering::Relaxed);
                            active_change.store(to_store, Ordering::Relaxed);
                        },
                        // Если клавиша равна клавише выходи из программы то угадайте что мы делаем
                        k if k == EXIT_BUTTON => {
                            process::exit(0);
                        },
                        // Остальные кнопки нас мало волнуют так что мы их пропускаем
                        _ => (),
                    }
                },
                // Ровно как и остальные события
                _ => (),
            }
        }
    });

    // метод `join` нужен чтобы наши функции не прекратили свою работу раньше чем функция `main`
    clicker.join().unwrap();
    toggle_clicker.join().unwrap();
}
