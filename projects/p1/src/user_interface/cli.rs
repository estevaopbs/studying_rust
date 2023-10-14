use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct UserInput {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Insert
    Insert(InsertCommand),
    ///Insert File
    InsertFile(InsertFileCommand),
    ///Insert Set
    InsertSet(InsertSetCommand),
    /// Delete
    Delete(DeleteCommand),
    /// Restore
    Restore(RestoreCommand),
    /// Show
    Show(ShowCommand),
    /// Undo
    Undo(UndoCommand),
    /// Redo
    Redo(RedoCommand),
    /// Export
    Export(ExportCommand),
    /// Alter
    Alter(AlterCommand),
    /// Copy
    Copy(CopyCommand),
    /// Balance
    Balance(BalanceCommand),
    /// SQLView
    SQLView(SqlCommand),
    /// Timezone
    DefaultTimezone(DefaultTimezoneCommand),
}

#[derive(Debug, Args)]
struct DefaultTimezoneCommand {
    /// Set Default
    #[clap(short, long, value_parser)]
    set: Option<i32>,
    /// Get Default
    #[clap(short, long)]
    get: bool,
}

#[derive(Debug, Parser)]
struct SqlCommand {
    /// Query
    #[clap(value_parser)]
    query: String,
}

#[derive(Debug, Args)]
struct InsertCommand {
    /// The name of the bill
    #[clap(short, long, value_parser)]
    name: Option<String>,
    /// The amount of items
    #[clap(short, long, value_parser)]
    amount: Option<u32>,
    /// The value of the bill
    #[clap(short, long, value_parser)]
    value: Option<f32>,
    /// Datetime
    #[clap(short, long, value_parser)]
    datetime: Option<String>,
    /// Currency
    #[clap(short, long, value_parser)]
    currency: Option<String>,
    /// Recipient
    #[clap(short, long, value_parser)]
    recipient: Option<String>,
    /// Situation
    #[clap(short, long, value_parser)]
    situation: Option<String>,
    /// Active
    #[clap(short, long)]
    deactivate: bool,
    /// Verbose
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Args)]
struct InsertFileCommand {
    /// Header
    #[clap(short = 'H', long)]
    header: bool,
    /// Path
    #[clap(short, long)]
    path: String,
}

#[derive(Debug, Args)]
struct InsertSetCommand {
    /// Colsep
    #[clap(short, long, value_parser, default_value = ",")]
    colsep: String,
    /// Rowsep
    #[clap(short, long, value_parser, default_value = ";")]
    rowsep: String,
    /// Header
    #[clap(short, long)]
    header: bool,
    /// Set
    set: String,
}

#[derive(Debug, Args)]
pub struct FilterSubcommand {
    /// The ID of the bill
    #[clap(short, long, value_parser)]
    pub id: Option<u32>,
    /// The name of the bill
    #[clap(short, long, value_parser)]
    pub name: Option<String>,
    /// The value of the bill
    #[clap(short, long, value_parser)]
    pub value: Option<f32>,
    /// The amount of the bill
    #[clap(short, long, value_parser)]
    pub amount: Option<String>,
    /// Datetime
    #[clap(short, long, value_parser)]
    pub datetime: Option<String>,
    /// Currency
    #[clap(short, long, value_parser)]
    pub currency: Option<String>,
    /// Recipient
    #[clap(short, long, value_parser)]
    pub recipient: Option<String>,
    /// Situation
    #[clap(short, long, value_parser)]
    pub situation: Option<String>,
    /// Orderby
    #[clap(short = 'o', long, value_parser)]
    pub orderby: Option<String>,
    /// Offset
    #[clap(short = 'O', long, value_parser)]
    pub limit: Option<u32>,
    /// Fetch
    #[clap(short, long, value_parser)]
    pub fetch: Option<u32>,
}

#[derive(Debug, Args)]
struct DeleteCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// Hard
    #[clap(short = 'H', long)]
    hard: bool,
    /// Active
    #[clap(short = 'A', long, value_enum, default_value = "Active")]
    active: Active,
}

#[derive(Debug, Args)]
struct ShowCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// Active
    #[clap(short = 'A', long, value_enum, default_value = "active")]
    active: Active,
    /// On Timezone
    #[clap(short, long, value_parser)]
    on_timezone: Option<i32>,
}

#[derive(Debug, Args)]
struct RestoreCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    #[clap(short = 'A', long, value_enum, default_value = "active")]
    active: Active,
}

#[derive(Debug, Args)]
struct UndoCommand {
    /// n
    #[clap(short, long, value_parser)]
    n: Option<u32>,
}

#[derive(Debug, Args)]
struct RedoCommand {
    /// n
    #[clap(short, long, value_parser)]
    n: Option<u32>,
}

#[derive(Debug, Args)]
struct ExportCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// Path
    #[clap(short, long, value_parser)]
    path: Option<String>,
    /// Columns
    #[clap(short = 'C', long, value_parser)]
    columns: Option<String>,
}

#[derive(Debug, Args)]
struct AlterCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// Columns
    #[clap(short = 'C', long, value_parser)]
    columns: String,
    /// New values
    #[clap(short = 'V', long, value_parser)]
    values: String,
    /// Active
    #[clap(short = 'A', long)]
    active: bool,
}

#[derive(Debug, Args)]
struct CopyCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// n
    #[clap(short = 'N', value_parser)]
    n: Option<u32>,
}

#[derive(Debug, Args)]
struct BalanceCommand {
    /// Where
    #[clap(flatten)]
    filter: FilterSubcommand,
    /// Active
    #[clap(short = 'A', long, value_enum, default_value = "Active")]
    active: Active,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Active {
    Active,
    Inactive,
    All,
}
