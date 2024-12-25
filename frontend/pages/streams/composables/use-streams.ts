import { Api } from "~/api/client";
import type { Row } from "./use-streams.types";
import type { Service } from "~/types/service";

export function useStreams() {
  const api = new Api("http://localhost:2137");

  async function getLastStreams(limit: number) {
    try {
      const resp = await api.getLastStreams({ limit });

      return {
        rows: resp.data.stream_with_packets.map((streamWithPackets) => {
          return {
            streamID: streamWithPackets.stream.id,
            serviceName: streamWithPackets.stream.service_name,
            servicePort: streamWithPackets.stream.service_port,
            rules: streamWithPackets.stream.rules,
            at: streamWithPackets.stream.started_at,
            packets: streamWithPackets.packets,
          } as Row;
        }),
      };
    } catch (e) {
      throw e as Error;
    }
  }

  async function getStreams(service_ids: number[], last_stream_id: number) {
    try {
      const resp = await api.getStreamsByServiceIDs({
        service_ids,
        last_stream_id,
      });

      return resp.data.stream_with_packets.map((streamWithPackets) => {
        return {
          streamID: streamWithPackets.stream.id,
          serviceName: streamWithPackets.stream.service_name,
          servicePort: streamWithPackets.stream.service_port,
          rules: streamWithPackets.stream.rules,
          at: streamWithPackets.stream.started_at,
          packets: streamWithPackets.packets,
        } as Row;
      });
    } catch (e) {
      throw e as Error;
    }
  }

  async function getServices() {
    try {
      const res = await api.getServices();

      return res.data.services_with_rules.map((serviceWithRules) => {
        return {
          id: serviceWithRules.service.id,
          name: serviceWithRules.service.name,
          port: serviceWithRules.service.port,
        } as Service;
      });
    } catch (e) {
      throw e as Error;
    }
  }

  return {
    getLastStreams,
    getStreams,
    getServices,
  };
}
