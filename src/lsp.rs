use tower_lsp::lsp_types::{InitializeParams, InitializeResult, InitializedParams};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::jsonrpc::Result;
use tokio::io::{stdin, stdout};

pub struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        // no-op
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub async fn start() -> anyhow::Result<()> {
    let (stdin, stdout) = (stdin(), stdout());
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
