interface Args {
  [key: string]: string;
}

interface Result {
  [key: string]: string;
}

class API {
  private url_base: string;

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

  public async read_query(
    query: string,
    args: Args,
  ): Promise<Result[]> {
    const res: string[][] = await fetch(`${this.url_base}/read/${query}`)
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
  }
  public async write_query(query: string, args: Args) {
    return await fetch(`${this.url_base}/write/${query}`, {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(args),
    });
  }
}

export { API };
