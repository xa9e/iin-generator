use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Приложение для генерации валидных ИИН", long_about = None)]
struct Args {
    /// Генерация ИИН для мужчин
    #[arg(short = 'm', long = "male")]
    male: bool,

    /// Генерация ИИН для женщин
    #[arg(short = 'f', long = "female")]
    female: bool,

    // todo: добавить вариант для генерации иин иностранцев
    /// Дата начала в формате DD-MM-YYYY
    #[arg(short = 's', long = "start", default_value = "01-01-1995")]
    start: String,

    /// Дата окончания в формате DD-MM-YYYY
    #[arg(short = 'e', long = "end", default_value = "05-01-2000")]
    end: String,

    /// Количество ИИН, генерируемых в день
    #[arg(short = 't', long = "take", default_value_t = 50)]
    take_in_a_day: u16,
}

pub fn run() {
    let args = Args::parse();

    if !args.male && !args.female {
        eprintln!("Укажите хотя бы один флаг: -m для мужчин или -f для женщин");
        return;
    }

    if args.male {
        super::gen(&args.start, &args.end, true, args.take_in_a_day);
    }

    if args.female {
        super::gen(&args.start, &args.end, false, args.take_in_a_day);
    }
}
