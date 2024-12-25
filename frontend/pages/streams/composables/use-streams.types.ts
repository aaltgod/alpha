import type {
  GetLastStreamsResponsePacketDto,
  GetLastStreamsResponseRuleDto,
} from "~/api/client";

export type Packet = GetLastStreamsResponsePacketDto;
export type Rule = GetLastStreamsResponseRuleDto;

export type Row = {
  streamID: number;
  serviceName: string;
  servicePort: number;
  rules: Rule[];
  at: string;
  packets: Packet[];
};
