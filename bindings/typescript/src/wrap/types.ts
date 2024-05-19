// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  CoreClient,
  InvokeResult,
  Uri,
} from "@polywrap/core-js";
import { PolywrapClient } from "@polywrap/client-js";

export type UInt = number;
export type UInt8 = number;
export type UInt16 = number;
export type UInt32 = number;
export type Int = number;
export type Int8 = number;
export type Int16 = number;
export type Int32 = number;
export type Bytes = Uint8Array;
export type BigInt = string;
export type BigNumber = string;
export type Json = string;
export type String = string;
export type Boolean = boolean;

/// Imported Enums START ///

/* URI: "fs/../wrap/build" */
export enum Tiktoken_TokenizerEnum {
  Cl100kBase,
  P50kBase,
  R50kBase,
  P50kEdit,
  Gpt2,
  Custom,
}

export type Tiktoken_TokenizerString =
  | "Cl100kBase"
  | "P50kBase"
  | "R50kBase"
  | "P50kEdit"
  | "Gpt2"
  | "Custom"

export type Tiktoken_Tokenizer = Tiktoken_TokenizerEnum | Tiktoken_TokenizerString;

/// Imported Enums END ///

/// Imported Modules START ///

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_getContextSize {
  model: Types.String;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_getTokenizer {
  model: Types.String;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_createCustomBpe {
  encoder: Map<Types.String, Types.UInt>;
  specialTokensEncoder: Map<Types.String, Types.UInt>;
  pattern: Types.String;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_encodeOrdinary {
  tokenizer: Types.Tiktoken_Tokenizer;
  text: Types.String;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_encode {
  tokenizer: Types.Tiktoken_Tokenizer;
  text: Types.String;
  allowedSpecial: Array<Types.String>;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_encodeWithSpecialTokens {
  tokenizer: Types.Tiktoken_Tokenizer;
  text: Types.String;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_decode {
  tokenizer: Types.Tiktoken_Tokenizer;
  tokens: Array<Types.UInt>;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_splitByToken {
  tokenizer: Types.Tiktoken_Tokenizer;
  text: Types.String;
  useSpecialTokens: Types.Boolean;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_splitByTokenOrdinary {
  tokenizer: Types.Tiktoken_Tokenizer;
  text: Types.String;
  bpe?: Array<Types.UInt8> | null;
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_r50kBase {
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_p50kBase {
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_p50kEdit {
}

/* URI: "fs/../wrap/build" */
export interface Tiktoken_Module_Args_cl100kBase {
}

/* URI: "fs/../wrap/build" */
export class Tiktoken {
  protected _defaultClient: CoreClient;
  protected _defaultUri: string;
  protected _defaultEnv?: Record<string, unknown>;

  constructor(
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ) {
    this._defaultClient = this._getClient(client);
    this._defaultEnv = this._getEnv(env);
    this._defaultUri = this._getUri(uri);
  }

  public get client(): CoreClient {
    return this._defaultClient;
  }

  public get uri(): string {
    return this._defaultUri;
  }

  public get env(): Record<string, unknown> | undefined {
    return this._defaultEnv;
  }

  private _getClient(client?: CoreClient): CoreClient {
    return client || this._defaultClient || this._getDefaultClient();
  }

  private _getUri(uri?: string): string {
    return uri || this._defaultUri || this._getDefaultUri();
  }

  private _getEnv(env?: Record<string, unknown>): Record<string, unknown> | undefined {
    return env || this._defaultEnv || this._getDefaultEnv();
  }

  protected _getDefaultClient(): CoreClient {
    return new PolywrapClient();
  }
  protected _getDefaultUri(): string {
    return "fs/../wrap/build";
  }
  protected _getDefaultEnv(): Record<string, unknown> | undefined {
    return undefined;
  }

  async getContextSize(
    args: Tiktoken_Module_Args_getContextSize,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Types.UInt>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Types.UInt>({
      uri: Uri.from(_uri),
      method: "getContextSize",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async getTokenizer(
    args: Tiktoken_Module_Args_getTokenizer,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Types.Tiktoken_Tokenizer | null>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Types.Tiktoken_Tokenizer | null>({
      uri: Uri.from(_uri),
      method: "getTokenizer",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async createCustomBpe(
    args: Tiktoken_Module_Args_createCustomBpe,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt8>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt8>>({
      uri: Uri.from(_uri),
      method: "createCustomBpe",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async encodeOrdinary(
    args: Tiktoken_Module_Args_encodeOrdinary,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt>>({
      uri: Uri.from(_uri),
      method: "encodeOrdinary",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async encode(
    args: Tiktoken_Module_Args_encode,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt>>({
      uri: Uri.from(_uri),
      method: "encode",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async encodeWithSpecialTokens(
    args: Tiktoken_Module_Args_encodeWithSpecialTokens,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt>>({
      uri: Uri.from(_uri),
      method: "encodeWithSpecialTokens",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async decode(
    args: Tiktoken_Module_Args_decode,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Types.String>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Types.String>({
      uri: Uri.from(_uri),
      method: "decode",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async splitByToken(
    args: Tiktoken_Module_Args_splitByToken,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.String>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.String>>({
      uri: Uri.from(_uri),
      method: "splitByToken",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async splitByTokenOrdinary(
    args: Tiktoken_Module_Args_splitByTokenOrdinary,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.String>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.String>>({
      uri: Uri.from(_uri),
      method: "splitByTokenOrdinary",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async r50kBase(
    args: Tiktoken_Module_Args_r50kBase,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt8>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt8>>({
      uri: Uri.from(_uri),
      method: "r50kBase",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async p50kBase(
    args: Tiktoken_Module_Args_p50kBase,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt8>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt8>>({
      uri: Uri.from(_uri),
      method: "p50kBase",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async p50kEdit(
    args: Tiktoken_Module_Args_p50kEdit,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt8>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt8>>({
      uri: Uri.from(_uri),
      method: "p50kEdit",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async cl100kBase(
    args: Tiktoken_Module_Args_cl100kBase,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.UInt8>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.UInt8>>({
      uri: Uri.from(_uri),
      method: "cl100kBase",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };
};

/// Imported Modules END ///
