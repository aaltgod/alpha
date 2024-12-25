import axios, { AxiosError, type AxiosInstance } from "axios";
import { string } from "zod";

export interface GetServicesRequestDto {}

export interface GetServicesResponseServiceRuleDto {
  id: number;
  name: string;
  packet_direction: string;
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
  packet_direction: string;
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

export interface GetLastStreamsRequestDto {
  limit: number;
}

export interface GetLastStreamsResponseRuleDto {
  id: number;
  name: string;
  packet_direction: string;
  regexp: string;
  color: string;
}

export interface GetLastStreamsResponseStreamDto {
  id: number;
  service_name: string;
  service_port: number;
  rules: GetLastStreamsResponseRuleDto[];
  started_at: string;
  ended_at: string;
}

export interface GetLastStreamsResponseRuleWithBordersDto {
  rule: GetLastStreamsResponseRuleDto;
  start: number;
  end: number;
}

export interface GetLastStreamsResponseTextWithColorDto {
  text: string;
  color: string;
}

export interface GetLastStreamsResponsePacketDto {
  payload: GetLastStreamsResponseTextWithColorDto[];
  direction: string;
  at: string;
}

export interface GetLastStreamsResponseStreamWithPacketsDto {
  stream: GetLastStreamsResponseStreamDto;
  packets: GetLastStreamsResponsePacketDto[];
}

export interface GetLastStreamsResponseDto {
  stream_with_packets: GetLastStreamsResponseStreamWithPacketsDto[];
}

export interface GetStreamsByServiceIDsRequest {
  service_ids: number[];
  last_stream_id: number;
}

export interface GetStreamsByServiceIDsResponse {
  stream_with_packets: GetLastStreamsResponseStreamWithPacketsDto[];
}

export class Api {
  private axiosInstance: AxiosInstance;

  constructor(baseURL: string) {
    this.axiosInstance = axios.create({
      baseURL: baseURL,
    });
  }

  upsertService(body: UpsertServiceRequestDto) {
    return this.axiosInstance.request<UpsertServiceResponseDto>({
      url: "/upsert-service",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  getServices() {
    return this.axiosInstance.request<GetServicesResponseDto>({
      url: "/get-services",
      responseType: "json",
      method: "GET",
    });
  }

  deleteService(body: DeleteServiceRequestDto) {
    return this.axiosInstance.request<DeleteServiceResponseDto>({
      url: "/delete-service",
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  createRule(body: CreateRuleRequestDto) {
    return this.axiosInstance.request<CreateRuleResponseDto>({
      url: "/create-rule",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  updateRule(body: UpdateRuleRequestDto) {
    return this.axiosInstance.request<UpdateRuleResponseDto>({
      url: "/update-rule",
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  deleteRule(body: DeleteRuleRequestDto) {
    return this.axiosInstance.request<DeleteRuleResponseDto>({
      url: "/delete-rule",
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  getRules() {
    return this.axiosInstance.request<GetRulesResponseDto>({
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

  getLastStreams(body: GetLastStreamsRequestDto) {
    return this.axiosInstance.request<GetLastStreamsResponseDto>({
      url: "/get-last-streams",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }

  getStreamsByServiceIDs(body: GetStreamsByServiceIDsRequest) {
    return this.axiosInstance.request<GetStreamsByServiceIDsResponse>({
      url: "/get-streams-by-service-ids",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      responseType: "json",
      data: JSON.stringify(body),
    });
  }
}
