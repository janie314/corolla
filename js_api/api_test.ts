import { Corolla } from "../js_api/index";
import { $ } from "bun";
import { afterAll, beforeAll, expect, test } from "bun:test";

async function cleanup() {
  await $`rm -rf ${import.meta.dir}/../tmp`;
  await $`mkdir -p ${import.meta.dir}/../tmp`;
}

beforeAll(cleanup);
afterAll(cleanup);

const corolla = new Corolla("http://127.0.0.1:50000", "/test");

const write01 = corolla.make_write_query<
  { c: string }
>("write01");

const read01 = corolla.make_read_query<
  {},
  { id: string; c: string }
>("read01");

test("write query", async () => {
  let res = await write01({ c: "carrot" });
  expect(res.ok).toBeTrue;
  res = await write01({ c: "tomato" });
  expect(res.ok).toBeTrue;
  res = await write01({ c: "squash" });
  expect(res.ok).toBeTrue;
});

test("read query", async () => {
  const rows = await read01({});
  expect(rows !== null);
  if (rows !== null) {
    expect(rows[0].c).toBe("carrot");
    expect(rows[1].c).toBe("tomato");
    expect(rows[2].c).toBe("squash");
  }
});
