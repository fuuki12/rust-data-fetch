import * as wasm from "../pkg/rust_data_fetch";

export default class TsAPI {
  private fetcher: typeof wasm;

  constructor() {
    this.fetcher = wasm;
  }

  useQuery(url: string, query: string): Promise<any> {
    return this.fetcher.use_query(url, query);
  }
}
