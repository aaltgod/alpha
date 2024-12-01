import axios, { AxiosError, type AxiosInstance } from "axios";

export interface GetServicesRequestDto {}

export interface GetServicesResponseServiceRuleDto {
  id: number;
  name: string;
  regexp: string;
  color: string;
}

export interface GetServicesResponseServiceDto {
  id: number;
  name: string;
  port: number;
}

export interface GetServicesResponseServiceWithRulesDto {
  service: GetServicesResponseServiceDto;
  rules?: GetServicesResponseServiceRuleDto[];
}

export interface GetServicesResponseDto {
  services_with_rules: GetServicesResponseServiceWithRulesDto[];
}

export interface CreateRuleRequestDto {
  id: number;
  name: string;
  packet_direction: string;
  regexp: string;
  color: string;
}

export interface CreateRuleResponseDto {}

export interface UpdateRuleRequestDto {
  id: number;
  name: string;
  packet_direction: string;
  regexp: string;
  color: string;
}

export interface UpdateRuleResponseDto {}

export interface DeleteRuleRequestDto {
  rule_id: number;
}

export interface DeleteRuleResponseDto {}

export interface GetRulesResponseRuleDto {
  id: number;
  name: string;
  regexp: string;
  color: string;
}

export interface GetRulesResponseDto {
  rules: GetRulesResponseRuleDto[];
}

export interface UpsertServiceRequestServiceDto {
  id?: number;
  name: string;
  port: number;
}

export interface UpsertServiceRequestDto {
  service: UpsertServiceRequestServiceDto;
  rule_ids: number[];
}

export interface UpsertServiceResponseDto {}

export interface DeleteServiceRequestDto {
  service_id: number;
}

export interface DeleteServiceResponseDto {}

export interface UpsertServiceResponseDto {}

export interface DeleteServiceToRulesRequestDto {
  service_id: number;
  rule_ids: number[];
}

export interface DeleteServiceToRulesResponsetDto {}

export class Api {
  private axiosInstance: AxiosInstance;

  constructor(baseURL: string) {
    this.axiosInstance = axios.create({
      baseURL: baseURL,
    });
  }

  upsertService(body: UpsertServiceRequestDto) {
    return this.axiosInstance.request<UpsertServiceResponseDto, AxiosError>({
      url: "/upsert-service",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  getServices() {
    return this.axiosInstance.request<GetServicesResponseDto, AxiosError>({
      url: "/get-services",
      responseType: "json",
      method: "GET",
    });
  }

  deleteService(body: DeleteServiceRequestDto) {
    return this.axiosInstance.request<DeleteServiceResponseDto, AxiosError>({
      url: "/delete-service",
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  createRule(body: CreateRuleRequestDto) {
    return this.axiosInstance.request<CreateRuleResponseDto, AxiosError>({
      url: "/create-rule",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  updateRule(body: UpdateRuleRequestDto) {
    return this.axiosInstance.request<UpdateRuleResponseDto, AxiosError>({
      url: "/update-rule",
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  deleteRule(body: DeleteRuleRequestDto) {
    return this.axiosInstance.request<DeleteRuleResponseDto, AxiosError>({
      url: "/delete-rule",
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  getRules() {
    return this.axiosInstance.request<GetRulesResponseDto, AxiosError>({
      url: "/get-rules",
      responseType: "json",
      method: "GET",
    });
  }

  deleteServiceToRules(body: DeleteServiceToRulesRequestDto) {
    return this.axiosInstance.request<
      DeleteServiceToRulesResponsetDto,
      AxiosError
    >({
      url: "/delete-service-to-rules",
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }
}
