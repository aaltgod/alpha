import { Separator } from "radix-vue/namespaced";
import { defineComponent } from "vue";
import { Badge } from "~/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
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
import { useStreams } from "./composables/use-streams";
import type { Row } from "./composables/use-streams.types";
import { toast, Toaster } from "~/components/ui/toast";
import axios from "axios";
import type { Service } from "~/types/service";

export default defineComponent({
  name: "StreamsPage",
  setup() {
    const { getLastStreams, getStreams, getServices } = useStreams();

    const rows = shallowRef<Row[]>([]);
    const selectedRow = ref<Row>();
    const services = ref<Service[]>([]);

    const fetchStreams = async () => {
      try {
        const streams = await getStreams(
          services.value.map((s) => s.id),
          rows.value[0].streamID
        );

        streams.push(...rows.value);

        rows.value = streams;
      } catch (error) {
        if (axios.isAxiosError(error)) {
          toast({
            title: "Error",
            variant: "destructive",
            description: error.response?.data || error.message,
          });
        }
      }
    };

    let intervalID: string | number | NodeJS.Timeout | undefined;

    onMounted(async () => {
      try {
        rows.value = (await getLastStreams(3000)).rows;
        selectedRow.value = rows.value[0];
      } catch (error) {
        if (axios.isAxiosError(error)) {
          toast({
            title: "Error",
            variant: "destructive",
            description: error.response?.data || error.message,
          });
        }
      }

      try {
        services.value = await getServices();
      } catch (error) {
        if (axios.isAxiosError(error)) {
          toast({
            title: "Error",
            variant: "destructive",
            description: error.response?.data || error.message,
          });
        }
      }

      intervalID = setInterval(fetchStreams, 5000);
    });

    onBeforeUnmount(() => {
      clearInterval(intervalID);
      rows.value = [];
    });

    const handleButtonClick = (row: Row) => {
      selectedRow.value = row;
    };

    return () => (
      <div class="grid auto-rows-max items-start gap-4">
        <Toaster />

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
                    {rows.value?.map((row) => (
                      <TableRow
                        key={row.streamID}
                        class="w-screen"
                        onClick={() => {
                          handleButtonClick(row);
                        }}
                        isSelected={
                          row.streamID === selectedRow.value?.streamID
                        }
                      >
                        <TableCell class="hidden sm:table-cell">
                          {row.streamID}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.serviceName}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.servicePort}
                        </TableCell>
                        <TableCell class="hidden sm:table-cell">
                          {row.rules.map((rule) => (
                            <Badge
                              class="text-xs"
                              style={`background-color: ${rule.color};`}
                            >
                              {rule.name}
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
                  № {selectedRow.value?.streamID}
                </CardTitle>
                <CardDescription>
                  Количестов пакетов: {selectedRow.value?.packets.length}
                </CardDescription>
              </div>
            </CardHeader>
            <CardContent class="p-6 text-sm">
              <ScrollArea class="h-screen">
                {selectedRow.value?.packets.map((packet) => (
                  <div>
                    <div class="border-b">
                      <div class="grid gap-3">
                        <div class="font-semibold">
                          {packet.at} {packet.direction}
                        </div>
                        <div class="flex-1 whitespace-pre-wrap p-4 text-sm">
                          {packet.payload.map((textWithColor) => (
                            <span
                              style={`background-color: ${textWithColor.color}`}
                            >
                              {textWithColor.text}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                    <Separator class="my-4" />
                  </div>
                ))}
              </ScrollArea>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  },
});
