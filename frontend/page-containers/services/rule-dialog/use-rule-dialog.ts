import { Api } from "~/api/client";
import type { CreateRule, UpdateRule } from "./types";

export function useRuleSidebar() {
  const api = new Api("http://localhost:2137");

  async function createRule(data: CreateRule) {
    try {
      await api.createRule({
        ...data,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  async function updateRule(data: UpdateRule) {
    try {
      await api.updateRule({
        ...data,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  async function deleteRule(ruleID: number) {
    try {
      await api.deleteRule({
        rule_id: ruleID,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  return {
    createRule,
    updateRule,
    deleteRule,
  };
}
