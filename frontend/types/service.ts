import type { Rule } from "./rule";

export type Service = {
  id: number;
  name: string;
  port: number;
  rules?: Rule[];
};
