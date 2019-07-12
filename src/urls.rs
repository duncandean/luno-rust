pub struct UrlMaker {
    api_base: reqwest::Url,
}

impl UrlMaker {
    /// Convenience constructor for UrlMaker.
    pub fn new(api_base: String) -> UrlMaker {
        let url = reqwest::Url::parse(&api_base).unwrap();
        UrlMaker { api_base: url }
    }

    /// Append a path to the API root
    fn build_url(&self, path: &str) -> reqwest::Url {
        self.api_base.join(path).unwrap()
    }

    /// Build https://api.mybitx.com/api/1/ticker?pair=...
    pub fn ticker(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("ticker");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/tickers
    pub fn tickers(&self) -> reqwest::Url {
        self.build_url("tickers")
    }

    /// Build https://api.mybitx.com/api/1/orderbook_top?pair=...
    pub fn orderbook_top(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("orderbook_top");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/orderbook?pair=...
    pub fn orderbook(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("orderbook");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/trades?pair=...
    pub fn trades(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("trades");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    // Build https://api.mybitx.com/api/1/accounts
    pub fn accounts(&self) -> reqwest::Url {
        self.build_url("accounts")
    }

    // Build https://api.mybitx.com/api/1/balance
    pub fn balance(&self) -> reqwest::Url {
        self.build_url("balance")
    }

    // Build https://api.mybitx.com/api/1/account/:id/transactions
    pub fn transactions(&self, account_id: &str, min_row: u64, max_row: u64) -> reqwest::Url {
        let mut url = self.accounts();
        url.path_segments_mut()
            .unwrap()
            .extend(&[account_id, "transactions"]);
        url.query_pairs_mut()
            .append_pair("min_row", &min_row.to_string())
            .append_pair("max_row", &max_row.to_string());
        url
    }

    // Build https://api.mybitx.com/api/1/account/:id/pending
    pub fn pending_transactions(&self, account_id: &str) -> reqwest::Url {
        let mut url = self.accounts();
        url.path_segments_mut()
            .unwrap()
            .extend(&[account_id, "pending"]);
        url
    }

    // Build https://api.mybitx.com/api/1/listorders
    pub fn list_orders(&self) -> reqwest::Url {
        self.build_url("listorders")
    }

    // Build https://api.mybitx.com/api/1/postorder
    pub fn post_order(&self) -> reqwest::Url {
        self.build_url("postorder")
    }

    // Build https://api.mybitx.com/api/1/marketorder
    pub fn market_order(&self) -> reqwest::Url {
        self.build_url("marketorder")
    }

    // Build https://api.mybitx.com/api/1/stoporder
    pub fn stop_order(&self) -> reqwest::Url {
        self.build_url("stoporder")
    }

    pub fn orders(&self, order_id: &str) -> reqwest::Url {
        let mut url = self.build_url("orders");
        url.path_segments_mut().unwrap().extend(&[order_id]);
        url
    }

    pub fn list_trades(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("listtrades");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }
}
