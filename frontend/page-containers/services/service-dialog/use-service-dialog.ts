import { Api } from "~/api/client";
import type { DeleteServiceToRules, UpsertService } from "./types";

export function useServiceDialog() {
  const api = new Api("http://localhost:2137");

  async function upsertService(data: UpsertService) {
    try {
      await api.upsertService({
        service: data.service,
        rule_ids: data.rule_ids,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  async function deleteService(serviceID: number) {
    try {
      await api.deleteService({
        service_id: serviceID,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  async function deleteServiceToRules(data: DeleteServiceToRules) {
    try {
      await api.deleteServiceToRules({
        service_id: data.service_id,
        rule_ids: data.rule_ids,
      });
    } catch (e) {
      throw e as Error;
    }
  }

  return {
    upsertService,
    deleteService,
    deleteServiceToRules,
  };
}
