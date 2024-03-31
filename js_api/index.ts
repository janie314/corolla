class Corolla {
  private url_base: string;
  private server: string;

  /**
   * @param url_base The Corolla server's URL base, e.g. "/application". Must start with "/". Empty parameter will default to "/". Trailing slashes will be trimmed.
   */
  public constructor(server: string = "", url_base: string = "/") {
    url_base = url_base.replace(/\/+$/, "");
    if (url_base.length === 0) {
      url_base = "/";
    }
    this.server = server;
    this.url_base = url_base;
  }

  /**
   * @param query The name of the Corolla read query.
   * @param args Key-value map of query arguments.
   * @returns The query's SQL results.
   */
  public make_read_query<Args extends { [key: string]: string }, Result>(
    query: string,
  ) {
    return async (
      args: Args,
    ): Promise<Result[]> => {
      const url_query_args = args === undefined
        ? ""
        : "?" + new URLSearchParams(args);
      const res: string[][] = await fetch(
        `${this.server}${this.url_base}/read/${query}${url_query_args}`,
      )
        .then((
          r,
        ) => r.json());
      if (res.length === 0) {
        return [];
      }
      const headers = res[0];
      return res.slice(1).map((row): Result => {
        return Object.fromEntries(
          headers.map((key: string, i: number) => [key, row[i]]),
        ) as Result;
      });
    };
  }

  /**
   * @param query The name of the Corolla query query.
   * @param args Key-value map of query arguments.
   */
  public make_write_query<Args extends { [key: string]: string }>(
    query: string,
  ) {
    return async (args: Args) => {
      return await fetch(`${this.server}${this.url_base}/write/${query}`, {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: args === undefined ? "{}" : JSON.stringify(args),
      });
    };
  }
}

export { Corolla };
