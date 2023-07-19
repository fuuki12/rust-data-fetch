import * as wasm from "../pkg/rust_data_fetch_bg.wasm";

export default class TsAPI {
  private fetcher: wasm.Fetcher;

  constructor(url: string) {
    this.fetcher = new wasm.Fetcher(url);
  }

  useQuery(query: string): Promise<string> {
    return this.fetcher.use_query(query);
  }
}
