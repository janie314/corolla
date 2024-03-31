interface Args {
  [key: string]: string;
}

interface Result {
  [key: string]: string;
}

class API {
  private url_base: string;

  /**
   * @param url_base The Corolla server's URL base, e.g. "/application". Must be empty (default) or start with "/".
   */
  public constructor(url_base: string = "") {
    if (
      url_base.length !== 0 && !/^\//.test(url_base) && !/\/$/.test(url_base)
    ) {
      url_base = "";
      console.error(
        `bad url_base ${url_base} passed to corolla API constructor. defaulting to ''`,
      );
    }
    this.url_base = url_base;
  }

  /**
   * @param query The name of the Corolla read query.
   * @param args Key-value map of query arguments.
   * @returns The query's SQL results.
   */
  public make_read_query<Result>(query: string) {
    return async (
      args?: Args,
    ): Promise<Result[]> => {
      const url_query_args = args === undefined
        ? ""
        : "?" + new URLSearchParams(args);
      const res: string[][] = await fetch(
        `${this.url_base}/read/${query}${url_query_args}`,
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
  public async make_write_query(query: string) {
    return async (args: Args) => {
      return await fetch(`${this.url_base}/write/${query}`, {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify(args),
      });
    };
  }
}

export { API };
