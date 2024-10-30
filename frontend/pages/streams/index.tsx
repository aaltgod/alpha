import { Copy, CreditCard, Truck } from "lucide-vue-next";
import { Separator } from "radix-vue/namespaced";
import { defineComponent, TransitionGroup } from "vue";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { ScrollArea } from "~/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";

type Packet = {
  direction: string;
  payload: string;
  at: string;
  flag_regexp: string;
  color: string;
};

type Row = {
  number: number;
  service: string;
  port: number;
  rules: string[];
  at: string;
  packets: Packet[];
};

export default defineComponent({
  name: "StreamsPage",
  setup() {
    const rows: Row[] = [];
    for (let i = 200; i > 0; i--) {
      const newRow: Row = {
        number: i,
        service: "service1",
        port: 7777,
        rules: ["flag_out", "red"],
        at: "13:00:12",
        packets: [
          {
            direction: "IN",
            payload: `GET /get-streams-by-service-ids HTTP/1.1\nContent-Type: application/json\nUser-Agent: PostmanRuntime/7.39.0\nAccept: */*\nPostman-Token: 833d5c87-2f2f-4a88-9d74-ca4a9026a2f4\nHost: localhost:2137\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive Content-Length: 28\n{ "service_ids": [1] }`,
            at: "13:00:12",
            flag_regexp: "FLAG==",
            color: "#color",
          },
          {
            direction: "OUT",
            payload:
              "HTTP/1.1 404 Not Found\ncontent-length: 0\ndate: Sun, 19 May 2024 13:47:01 GMT\n\n\n",
            at: "13:00:13",
            flag_regexp: "FLAG==",
            color: "#color",
          },
          {
            direction: "IN",
            payload: `GET /get-streams-by-service-ids HTTP/1.1\nContent-Type: application/json\nUser-Agent: PostmanRuntime/7.39.0\nAccept: */*\nPostman-Token: 833d5c87-2f2f-4a88-9d74-ca4a9026a2f4\nHost: localhost:2137\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive Content-Length: 28\n{ "service_ids": [1] }`,
            at: "13:00:14",
            flag_regexp: "FLAG==",
            color: "#color",
          },
        ],
      };
      rows.push(newRow);
    }

    const selectedRow = ref<Row>(rows[0]);

    const handleButtonClick = (row: Row) => {
      console.log(row);
      selectedRow.value = row;
    };

    return () => (
      <div class="grid auto-rows-max items-start gap-4">
        <div
          class="grid gap-4 h-screen sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-2"
          style="grid-template-columns: 500px 1fr"
        >
          <Card>
            <ScrollArea class="h-screen">
              <CardContent>
                <Table>
                  <TableHeader class="sticky top-0 bg-muted/100">
                    <TableRow>
                      <TableHead>№</TableHead>
                      <TableHead>Service</TableHead>
                      <TableHead>Port</TableHead>
                      <TableHead>Rules</TableHead>
                      <TableHead>At</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {rows.map((row) => (
                      <TableRow class="w-screen">
                        <button
                          onClick={() => {
                            handleButtonClick(row);
                          }}
                        >
                          <TableCell class="hidden sm:table-cell">
                            {row.number}
                          </TableCell>
                        </button>
                        <TableCell class="hidden sm:table-cell">
                          {row.service}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.port}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.rules.map((rule) => (
                            <Badge class="text-xs" variant="outline">
                              {rule}
                            </Badge>
                          ))}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.at}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </CardContent>
            </ScrollArea>
          </Card>
          <Card>
            <CardHeader class="flex flex-row items-start bg-muted/50">
              <div class="grid gap-0.5">
                <CardTitle class="group flex items-center gap-2 text-lg">
                  № {selectedRow.value.number}
                </CardTitle>
                <CardDescription>
                  Количестов пакетов: {selectedRow.value.packets.length}
                </CardDescription>
              </div>
            </CardHeader>
            <ScrollArea>
              <CardContent class="p-6 text-sm">
                {selectedRow.value.packets.map((packet) => (
                  <div>
                    <div class="border-b">
                      <div class="grid gap-3">
                        <div class="font-semibold">
                          {packet.at} {packet.direction}
                        </div>
                        <div class="flex-1 whitespace-pre-wrap p-4 text-sm">
                          {packet.payload}
                        </div>
                      </div>
                    </div>
                    <Separator class="my-4" />
                  </div>
                ))}
              </CardContent>
            </ScrollArea>
          </Card>
        </div>
      </div>
    );
  },
});
