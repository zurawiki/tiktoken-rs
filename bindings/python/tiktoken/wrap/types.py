# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from typing import Any, TypedDict, Optional
from enum import IntEnum

from polywrap import (
    Uri,
    Client,
    GenericMap,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    sys_bundle,
    web3_bundle
)


### Env START ###

### Env END ###

### Objects START ###

### Objects END ###

### Enums START ###
### Enums END ###

### Imported Objects START ###

### Imported Objects END ###

### Imported Enums START ###

# URI: "fs/../wrap/build" #
class TiktokenTokenizer(IntEnum):
    Cl100kBase = 0, "0", "Cl100kBase"
    P50kBase = 1, "1", "P50kBase"
    R50kBase = 2, "2", "R50kBase"
    P50kEdit = 3, "3", "P50kEdit"
    Gpt2 = 4, "4", "Gpt2"
    Custom = 5, "5", "Custom"

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj


### Imported Enums END ###

### Imported Modules START ###

# URI: "fs/../wrap/build" #
TiktokenModuleArgsGetContextSize = TypedDict("TiktokenModuleArgsGetContextSize", {
    "model": str,
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsGetTokenizer = TypedDict("TiktokenModuleArgsGetTokenizer", {
    "model": str,
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsCreateCustomBpe = TypedDict("TiktokenModuleArgsCreateCustomBpe", {
    "encoder": GenericMap[str, int],
    "specialTokensEncoder": GenericMap[str, int],
    "pattern": str,
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsEncodeOrdinary = TypedDict("TiktokenModuleArgsEncodeOrdinary", {
    "tokenizer": "TiktokenTokenizer",
    "text": str,
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsEncode = TypedDict("TiktokenModuleArgsEncode", {
    "tokenizer": "TiktokenTokenizer",
    "text": str,
    "allowedSpecial": list[str],
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsEncodeWithSpecialTokens = TypedDict("TiktokenModuleArgsEncodeWithSpecialTokens", {
    "tokenizer": "TiktokenTokenizer",
    "text": str,
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsDecode = TypedDict("TiktokenModuleArgsDecode", {
    "tokenizer": "TiktokenTokenizer",
    "tokens": list[int],
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsSplitByToken = TypedDict("TiktokenModuleArgsSplitByToken", {
    "tokenizer": "TiktokenTokenizer",
    "text": str,
    "useSpecialTokens": bool,
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsSplitByTokenOrdinary = TypedDict("TiktokenModuleArgsSplitByTokenOrdinary", {
    "tokenizer": "TiktokenTokenizer",
    "text": str,
    "bpe": Optional[list[int]],
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsR50kBase = TypedDict("TiktokenModuleArgsR50kBase", {
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsP50kBase = TypedDict("TiktokenModuleArgsP50kBase", {
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsP50kEdit = TypedDict("TiktokenModuleArgsP50kEdit", {
}, total=False)

# URI: "fs/../wrap/build" #
TiktokenModuleArgsCl100kBase = TypedDict("TiktokenModuleArgsCl100kBase", {
}, total=False)

# URI: "fs/../wrap/build" #
class Tiktoken:
    _default_client: Client
    _default_uri: Uri
    _default_env: Optional[Any]

    def __init__(
        self,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ):
        self._default_client = self._get_client(client)
        self._default_uri = self._get_uri(uri)
        self._default_env = self._get_env(env)

    def _get_client(self, client: Optional[Client]) -> Client:
        return client or getattr(self, "_default_client", None) or self._get_default_client()

    def _get_uri(self, uri: Optional[Uri]) -> Uri:
        return uri or getattr(self, "_default_uri", None) or self._get_default_uri() 

    def _get_env(self, env: Optional[Any]) -> Any:
        return env or getattr(self, "_default_env", None) or self._get_default_env()

    def _get_default_client(self) -> Client:
        config = (
            PolywrapClientConfigBuilder()
            .add_bundle(sys_bundle)
            .add_bundle(web3_bundle)
            .build()
        )
        return PolywrapClient(config)

    def _get_default_uri(self) -> Optional[Uri]:
        return Uri.from_str("fs/../wrap/build")

    def _get_default_env(self) -> Any:
        return None

    def get_context_size(
        self,
        args: TiktokenModuleArgsGetContextSize,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> int:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="getContextSize",
            args=args,
            env=_env,
        )

    def get_tokenizer(
        self,
        args: TiktokenModuleArgsGetTokenizer,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> Optional["TiktokenTokenizer"]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="getTokenizer",
            args=args,
            env=_env,
        )

    def create_custom_bpe(
        self,
        args: TiktokenModuleArgsCreateCustomBpe,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="createCustomBpe",
            args=args,
            env=_env,
        )

    def encode_ordinary(
        self,
        args: TiktokenModuleArgsEncodeOrdinary,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="encodeOrdinary",
            args=args,
            env=_env,
        )

    def encode(
        self,
        args: TiktokenModuleArgsEncode,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="encode",
            args=args,
            env=_env,
        )

    def encode_with_special_tokens(
        self,
        args: TiktokenModuleArgsEncodeWithSpecialTokens,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="encodeWithSpecialTokens",
            args=args,
            env=_env,
        )

    def decode(
        self,
        args: TiktokenModuleArgsDecode,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> str:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="decode",
            args=args,
            env=_env,
        )

    def split_by_token(
        self,
        args: TiktokenModuleArgsSplitByToken,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[str]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="splitByToken",
            args=args,
            env=_env,
        )

    def split_by_token_ordinary(
        self,
        args: TiktokenModuleArgsSplitByTokenOrdinary,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[str]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="splitByTokenOrdinary",
            args=args,
            env=_env,
        )

    def r50k_base(
        self,
        args: TiktokenModuleArgsR50kBase,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="r50kBase",
            args=args,
            env=_env,
        )

    def p50k_base(
        self,
        args: TiktokenModuleArgsP50kBase,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="p50kBase",
            args=args,
            env=_env,
        )

    def p50k_edit(
        self,
        args: TiktokenModuleArgsP50kEdit,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="p50kEdit",
            args=args,
            env=_env,
        )

    def cl100k_base(
        self,
        args: TiktokenModuleArgsCl100kBase,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[int]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="cl100kBase",
            args=args,
            env=_env,
        )

### Imported Modules END ###
