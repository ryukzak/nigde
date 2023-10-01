# nigde

## Description

REST Api service to run

## Project structure

The project consists of rust packages combined into a single workspace

**Packages:**

- nigde - Rest service, for controlling the processor via the API
- fpga_hps_programmator - Firmware loading into FPGA and other low-level interaction between HPS and FPGA

**Other packages:**

- simple_led_driver - simplest example of rust userspace driver

## Dependencies

- [axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio, Tower, and Hyper
- [libc](https://github.com/rust-lang/libc) - Raw bindings to platform APIs for Rust

## Build project

If you need to build for `arm-unknown-linux-gnueabihf` platform, use Dockerfile:

```bash
docker built -t nigde_arm_builder
docker run --rm -v .:/app nigde_arm_builder:latest
```

After completion, the build output will be placed to `./target/arm-unknown-linux-gnueabihf/release/nigde`

## Build led driver example

```bash
cd simple_led_driver
cargo build
```

---

## Документация
### Подключение платы

Плата питается от 5V, я подключал через круглый разъем
На SD карте уже есть ОС с Debian 10, я брал готовый образ (можно пойти другим путем и собрать свой).

К плате удобнее всего подключаться по ssh (креды root:root).
`ssh root@<de-10-nano-ip>`

#### Сеть

Если интернет к плате подключается через ETHERNET кабель, все должно завестись само.

Если планируется использовать wifi, можно подключиться к плате через serial-порт и настроить сеть.
У себя на ноутбуке для подключения по serial порту я использовал утилиту `minicom`

Кажется, есть еще другой способ настроить wifi - воткнуть SD карту с прошивкой к себе, чрутнуться и отредактировать конфиг для wifi.

#### MSEL пины

Там на плате есть какие-то маленькие переключатели. Их нужно выставить в правильное положение, иначе при прошивке будет выдаваться ошибка на команде `echo -n "abc.dtbo" > abc/path`
Подробнее описано в [Absolute beginner's guide to DE10-Nano](https://github.com/zangman/de10-nano/blob/master/docs/Flash-FPGA-from-HPS-running-Linux.md)

### Что было сделано

В основном, я почитал "Absolute beginner's guide to DE10-Nano" и помигал светодиодом. (см. ссылки \[1\])

Еще сделал небольшую штуку для загрузки прошивки в ПЛИС из хостовой системы. Оно есть в виде крейта `fpga_hps_programmator`. В принципе она только генерирует файлы для device tree и компилит их. Потом маунтит `configfs`, копирует туда нужные файлики и прошивает.
Для запуска есть `fpga_hps_programmator_bin` (название файла прошивки я там захардкодил)
Я делил код на крейты в воркспейсе для оптимизации скорости сборки

Переписал простенький драйвер из Absolute beginner's guide to DE10-Nano на раст, можно глянуть в `simple_led_driver/src/main.rs`

И еще сделал пайплайн для кросскомпиляции проекта сразу под архитектуру платы (бинарник выпадет в артефакт).

### Сборка в CI

Чуть подробнее про то как работал пайплайн.

- Ставятся пакеты `gcc-arm-linux-gnueabihf libc6-dev-armhf-cross`, которые нужны для сборки под arm
- Стягивается код
- Запускается проверка корректности md файлов (?)
- Кэш сборки нужен для двух вещей:
  - индекс crates.io очень долго загружается с нуля, если не использовать sparse-index (но на момент работы такая опция была только в nightly).
  - чтобы не пересобирать зависимости
- Ставится rustup, тулчейны
- Запускается rustfmt (проверка синтаксиса), тесты, clippy (тоже проверки кода)
- Сборка
- Выгрузка

### Полезные ссылки

1. Absolute beginner's guide to DE10-Nano - https://github.com/zangman/de10-nano/tree/master
2. minicom tutorial - https://soft-setup.ru/instrukcziya-po-ispolzovaniyu-minicom-v-linux/
