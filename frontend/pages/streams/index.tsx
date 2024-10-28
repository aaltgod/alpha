import { Copy, CreditCard, Truck } from "lucide-vue-next"
import { Separator } from "radix-vue/namespaced"
import { defineComponent } from "vue"
import { Badge } from "~/components/ui/badge"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "~/components/ui/card"
import { ScrollArea } from "~/components/ui/scroll-area"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table"

type Packet = {
  direction: string,
  payload: string,
  at: string,
  flag_regexp: string,
  color: string,
}

type Row = {
  number: number
  service: string
  port: number
  rules: string[]
  at: string 
  packets: Packet[]
}

export default defineComponent({
	name: 'StreamsPage',
	setup() {
    const rows: Row[] = [
    ]
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
              payload: "HTTP/1.1 404 Not Found\ncontent-length: 0\ndate: Sun, 19 May 2024 13:47:01 GMT\n\n\n",
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
            {
              direction: "OUT",
              payload: "HTTP/1.1 404 Not Found\ncontent-length: 0\ndate: Sun, 19 May 2024 13:47:01 GMT\n\n\n",
              at: "13:00:15",
              flag_regexp: "FLAG==",
              color: "#color",
            },
            {
              direction: "IN",
              payload: `GET /get-streams-by-service-ids HTTP/1.1\nContent-Type: application/json\nUser-Agent: PostmanRuntime/7.39.0\nAccept: */*\nPostman-Token: 833d5c87-2f2f-4a88-9d74-ca4a9026a2f4\nHost: localhost:2137\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive Content-Length: 28\n{ "service_ids": [1] }`,
              at: "13:00:16",
              flag_regexp: "FLAG==",
              color: "#color",
            },
            {
              direction: "OUT",
              payload: "HTTP/1.1 404 Not Found\ncontent-length: 0\ndate: Sun, 19 May 2024 13:47:01 GMT\n\n\n",
              at: "13:00:17",
              flag_regexp: "FLAG==",
              color: "#color",
            },
          ]
          
      };
      rows.push(newRow);

    }

    const selectedRow = ref<Row>(rows[0])

    const handleButtonClick = (row: Row) => {
      console.log(row)
      selectedRow.value = row
    }

		return () => (
			<div class="flex min-h-screen w-full flex-col bg-muted/70">
  			<div class="flex flex-col sm:gap-4 sm:py-3 sm:pl-2 sm:pr-2">
        	<div class="grid auto-rows-max items-start gap-4 md:gap-8 lg:col-span-2">
          	<div class="grid min-h-screen gap-4 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-2" style="grid-template-columns: 30% 70%">
              <ScrollArea class="h-screen flex">
                <Card>
                  <CardContent>
                    <Table>
                      <TableHeader>
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
                            <button onClick={() => {handleButtonClick(row)}}>
                              <TableRow>
                                <TableCell class="hidden sm:table-cell">
                                  {row.number}
                                </TableCell>
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
                            </button>
                          ))}
                      </TableBody>
                    </Table>
                  </CardContent>
                  </Card>
                </ScrollArea>
                <ScrollArea class="h-screen flex">
                  <Card>
                    <CardHeader class="flex flex-row items-start bg-muted/50">
                      <div class="grid gap-0.5">
                        <CardTitle class="group flex items-center gap-2 text-lg">
                          <Button
                            size="icon"
                            variant="outline"
                            class="h-6 w-6 opacity-0 transition-opacity group-hover:opacity-100"
                          >
                            <Copy class="h-3 w-3" />
                              <span class="sr-only">Copy Order ID</span>
                          </Button>
                        </CardTitle>
                        <CardDescription>Date: November 23, 2023</CardDescription>
                      </div>
                      <div class="ml-auto flex items-center gap-1">
                        <Button size="sm" variant="outline" class="h-8 gap-1">
                          <Truck class="h-3.5 w-3.5" />
                            <span class="lg:sr-only xl:not-sr-only xl:whitespace-nowrap">
                              Track Order
                            </span>
                        </Button>
                      </div>
                    </CardHeader>
                    <CardContent class="p-6 text-sm">
                      {selectedRow.value.packets.map((packet) => (
                        <div>
                          <div class="grid gap-3">
                            <div class="font-semibold">
                              {packet.at}
                            </div>
                            <div class="flex-1 whitespace-pre-wrap p-4 text-sm">
                              { packet.payload }
                            </div>
                          </div>
                          <Separator class="my-6" />
                        </div>
                      ))}
                    </CardContent>
                  </Card>
                </ScrollArea>
              </div>
            </div>
		      </div>
		    </div>
  		)
	  },
  })
