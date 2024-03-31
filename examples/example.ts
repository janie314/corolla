import { Corolla } from "../js_api/index";

const corolla = new Corolla("/test");
const write01 = corolla.make_write_query<
  { c: string }
>("read01");
await write01({ c: new Date().toISOString() });
const read01 = corolla.make_read_query<
  {},
  { id: string; c: string }
>("read01");
const res = await read01({});
console.log(res);
