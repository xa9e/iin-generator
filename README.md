# iin-generator
Генератор вордлистов с валидными ИИН

Установка в системный путь
```bash
sudo make install
``` 

Установка в пользовательскую директорию
```bash
cargo install --path .
```

```text
Usage: iin-generator [OPTIONS]

Options:
  -m, --male                  Генерация ИИН для мужчин
  -f, --female                Генерация ИИН для женщин
  -s, --start <START>         Дата начала в формате DD-MM-YYYY [default: 01-01-1995]
  -e, --end <END>             Дата окончания в формате DD-MM-YYYY [default: 05-01-2000]
  -t, --take <TAKE_IN_A_DAY>  Количество ИИН, генерируемых в день [default: 50]
  -h, --help                  Print help
  -V, --version               Print version
  ```
