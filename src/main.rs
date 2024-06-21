use std::path::{Component, PathBuf};

use clap::{Parser, Subcommand};
use fasthash::city::Hash32;
use fasthash::FastHash;

#[derive(Parser)]
#[command(
    name = "hburger",
    version = "0.1.0",
    about = "Turn strings into hashburgers"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Turn a string into a hashburger")]
    Hash {
        #[arg(help = "Directory to create breadcrumbs for")]
        input: String,

        #[command(flatten)]
        burgerize_args: BurgerizeArgs,
    },
    #[command(about = "Turn a path into a compressed series of hashburgers")]
    HashPath {
        #[arg(help = "The input path to transform")]
        input: PathBuf,

        #[arg(
            short,
            long,
            default_value = "2",
            help = "Number of components to keep from the start of the path"
        )]
        start_components: usize,

        #[arg(
            short,
            long,
            default_value = "2",
            help = "Number of tail components to keep from the end of the path"
        )]
        end_components: usize,

        #[arg(
            short,
            long,
            default_value = ":",
            help = "Character to use to divide start and end components"
        )]
        divider: char,

        #[command(flatten)]
        burgerize_args: BurgerizeArgs,
    },
}

#[derive(Parser)]
struct BurgerizeArgs {
    #[arg(
        short,
        long,
        default_value = "4",
        help = "The length of the hashburger's left bun"
    )]
    left_bun_length: usize,

    #[arg(
        short,
        long,
        default_value = "2",
        help = "The length of the hasburger's center hashpatty"
    )]
    center_hashpatty_length: usize,

    #[arg(
        short,
        long,
        default_value = "4",
        help = "The length of the hashburger's right bun"
    )]
    right_bun_length: usize,

    #[arg(
        short,
        long,
        help = "The character to use to pad hashburgers (defaults to no padding)"
    )]
    padding_char: Option<char>,
}

fn main() {
    let args = Args::parse();
    let stdout = match args.command {
        Commands::Hash {
            input,
            burgerize_args,
        } => burgerize(&input, &burgerize_args),
        Commands::HashPath {
            input,
            start_components,
            end_components,
            divider,
            burgerize_args: options,
        } => burgerize_path(input, start_components, end_components, divider, options),
    };

    println!("{}", stdout);
}

fn burgerize(string: &String, args: &BurgerizeArgs) -> String {
    let hashburger_length =
        args.left_bun_length + args.center_hashpatty_length + args.right_bun_length;

    if string.len() <= hashburger_length {
        return match args.padding_char {
            None => string.to_string(),
            Some(padding_char) => {
                let padding_length = hashburger_length - string.len();
                let padding = padding_char.to_string().repeat(padding_length);
                format!("{string}{padding}")
            }
        };
    }

    let left_bun = &string[..args.left_bun_length];
    let right_bun = &string[string.len() - args.right_bun_length..];

    let patty = &string[args.left_bun_length..string.len() - args.right_bun_length];
    let hashpatty = &Hash32::hash(patty.as_bytes()).to_string()[..args.center_hashpatty_length];

    let hashburger = format!("{left_bun}{hashpatty}{right_bun}");
    hashburger
}

fn burgerize_path(
    input: PathBuf,
    start_components: usize,
    end_components: usize,
    divider: char,
    burgerize_args: BurgerizeArgs,
) -> String {
    let burgerized_component_length = burgerize_args.left_bun_length
        + burgerize_args.center_hashpatty_length
        + burgerize_args.right_bun_length;
    let burgerized_path_length =
        (1 + burgerized_component_length) * (start_components + end_components);

    let mut result = String::with_capacity(burgerized_path_length);

    let components: Vec<Component> = if input.is_absolute() {
        result.push_str(Component::RootDir.as_os_str().to_str().unwrap());
        input.components().skip(1).collect()
    } else {
        input.components().collect()
    };

    if start_components + end_components >= components.len() {
        let burgerized_components = components.iter().map(|component| {
            burgerize(
                &component.as_os_str().to_str().unwrap().to_string(),
                &burgerize_args,
            )
        });
        result.push_str(&burgerized_components.collect::<PathBuf>().to_str().unwrap());
    } else {
        let start_components = components.iter().take(start_components);
        let end_components = components.iter().rev().take(end_components).rev();

        let burgerized_start_components = start_components.map(|component| {
            burgerize(
                &component.as_os_str().to_str().unwrap().to_string(),
                &burgerize_args,
            )
        });
        let burgerized_end_components = end_components.map(|component| {
            burgerize(
                &component.as_os_str().to_str().unwrap().to_string(),
                &burgerize_args,
            )
        });

        result.push_str(
            &burgerized_start_components
                .collect::<PathBuf>()
                .to_str()
                .unwrap(),
        );
        result.push(divider);
        result.push_str(
            &burgerized_end_components
                .collect::<PathBuf>()
                .to_str()
                .unwrap(),
        );
    };

    result
}
