use crate::{Cli, EtopError};

pub(crate) async fn print_command(args: Cli) -> Result<(), EtopError> {
    let mut etop_state = super::tui_command::create_etop_state(
        args.dataset,
        args.block,
        args.window,
        args.rpc,
        args.data_dir,
    )
    .await?;
    let queries = etop_state.create_missing_queries()?;
    for query in queries.into_iter() {
        let result = etop_state.query(query.clone()).await?;
        etop_state.warehouse.add_dataset(query.dataset(), result)?;
    }
    let queries = etop_state.create_missing_queries()?;
    for query in queries.into_iter() {
        let result = etop_state.query(query.clone()).await?;
        etop_state.warehouse.add_dataset(query.dataset(), result)?;
    }

    let queries = etop_state.create_missing_queries()?;
    for query in queries.into_iter() {
        let result = etop_state.query(query.clone()).await?;
        etop_state.warehouse.add_dataset(query.dataset(), result)?;
    }

    let (render_width, render_height) = term_size::dimensions().unwrap_or((80, 20));
    let s = etop_state.format_window(render_width, render_height)?;
    println!("{}", s);

    Ok(())
}
