impl DB {
    async fn init_db_info(&self) -> Result<(), Error> {
        self.write_raw_query("create table if not exists corolla_db_info (key text, value text);")
            .await?;
        self.write_raw_query(
            "insert into corolla_db_info values ('version', '{}'})",
            version2str(&SPEC_VERSION),
        )
    }
}
