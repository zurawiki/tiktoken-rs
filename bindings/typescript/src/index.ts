import { Tiktoken } from "./wrap";

async function main() {
  const tiktoken = new Tiktoken();

  console.log("encode...");
  const encodedResult = await tiktoken.encodeWithSpecialTokens({
    tokenizer: "P50kBase",
    text: "This is a test         with a lot of spaces"
  });

  if (!encodedResult.ok) {
    throw new Error("Encode failed");
  }
  const encoded = encodedResult.value;
  console.log(`encoded: ${encoded}`);


  console.log("decode...");
  const decodedResult = await tiktoken.decode({
    tokenizer: "P50kBase",
    tokens: encoded
  });

  if (!decodedResult.ok) {
    throw new Error("Decode failed");
  }

  const decoded = decodedResult.value;
  console.log(`decoded: ${decoded}`);

  if (decoded !== "This is a test         with a lot of spaces") {
    throw new Error("Decoded text is not equal to original text");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
