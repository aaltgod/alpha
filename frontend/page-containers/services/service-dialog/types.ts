import type {
  UpsertServiceRequestDto,
  DeleteServiceToRulesRequestDto,
} from "~/api/client";
import type { Rule } from "~/types/rule";

export type UpsertService = UpsertServiceRequestDto;
export type DeleteServiceToRules = DeleteServiceToRulesRequestDto;

export type RuleWithChecked = Rule & {
  checked: boolean;
};
