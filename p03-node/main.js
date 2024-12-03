import fs from "node:fs/promises";

async function part1() {
  const inputString = await fs.readFile("./input.txt", "utf8");
  const regexp = /mul\((\d{1,3}),(\d{1,3})\)/g;
  let sum = BigInt(0);
  for (const [, n1, n2] of inputString.matchAll(regexp)) {
    sum += BigInt(n1) * BigInt(n2);
  }

  console.log(`sum=${sum.toString()}`);
}

// Incorrect
async function part2() {
  const inputString =
    "do()" + (await fs.readFile("./input.txt", "utf8")) + "don't()";
  const regexpSegments = /(?<=do\(\))(.*?)(?=don't\(\))/g;
  const regexpInner = /mul\((\d{1,3}),(\d{1,3})\)/g;

  let sum = BigInt(0);

  for (const [inner] of inputString.matchAll(regexpSegments)) {
    for (const [, n1, n2] of inner.matchAll(regexpInner)) {
      sum += BigInt(n1) * BigInt(n2);
    }
  }

  console.log(`sum=${sum.toString()}`);
}

await part1();
await part2();
