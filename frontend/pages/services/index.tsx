import axios from "axios";
import { defineComponent } from "vue";
import { Api } from "~/api/client";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";

import { toast } from "~/components/ui/toast";
import Toaster from "~/components/ui/toast/Toaster.vue";
import { ServiceDialog } from "~/page-containers/services/service-dialog/service-dialog";
import { RuleDialog } from "~/page-containers/services/rule-dialog/rule-dialog";
import type { Rule } from "~/types/rule";
import type { Service } from "~/types/service";

export default defineComponent({
  name: "ServicesPage",
  setup() {
    const api = new Api("http://localhost:2137");

    const services = ref([]) as Ref<Array<Service>>;

    const getServices = async (): Promise<void> => {
      try {
        services.value = [];

        const res = await api.getServices();

        res.data.services_with_rules.map((serviceWithRules) => {
          services.value.push({
            id: serviceWithRules.service.id,
            name: serviceWithRules.service.name,
            port: serviceWithRules.service.port,
            rules: serviceWithRules.rules?.map((rule) => ({
              id: rule.id,
              name: rule.name,
              packetDirection: rule.packet_direction,
              regexp: rule.regexp,
              color: rule.color,
            })),
          });
        });
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

    const rules = ref([]) as Ref<Array<Rule>>;

    const getRules = async (): Promise<void> => {
      try {
        rules.value = [];

        const res = await api.getRules();

        res.data.rules.map((rule) => {
          rules.value.push({
            id: rule.id,
            name: rule.name,
            regexp: rule.regexp,
            color: rule.color,
            packetDirection: rule.packet_direction,
          });
        });
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

    onMounted(() => {
      getRules();
      getServices();
    });

    const handleSubmit = async () => {
      getServices();
      getRules();
    };

    const serviceToOpenDialog = ref<Service>();
    const ruleToOpenDialog = ref<Rule>();

    return () => (
      <div class="grid auto-rows-max items-start gap-4">
        <Toaster />

        <div
          class="grid gap-4 h-screen sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-2"
          style="grid-template-columns: 400px 1fr"
        >
          <Card>
            <CardHeader class="flex flex-row justify-between pb-3">
              <p>Rules</p>
              <Button
                onClick={() => {
                  ruleToOpenDialog.value = {
                    id: 0,
                    name: "",
                    regexp: "",
                    color: "",
                  } as Rule;
                }}
              >
                Create rule
              </Button>
            </CardHeader>
            <CardContent>
              <div class="flex gap-3 flex-wrap">
                {rules.value.map((rule) => (
                  <Card
                    key={rule.id}
                    style={`background-color: ${rule.color};`}
                    onClick={() => {
                      ruleToOpenDialog.value = rule;
                    }}
                  >
                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-3">
                      <CardTitle class="text-sm font-medium">
                        {rule.name}
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p class="text-xs text-muted-foreground">{rule.regexp}</p>
                    </CardContent>
                  </Card>
                ))}
              </div>
              {ruleToOpenDialog.value && (
                <RuleDialog
                  data={ruleToOpenDialog.value}
                  whenSubmitted={handleSubmit}
                  whenClose={() => {
                    ruleToOpenDialog.value = undefined;
                  }}
                ></RuleDialog>
              )}
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="flex flex-row justify-between pb-3">
              <p>Services</p>
              <Button
                onClick={() => {
                  serviceToOpenDialog.value = {
                    id: 0,
                    name: "",
                    rules: [],
                    port: 0,
                  } as Service;
                }}
              >
                Create service
              </Button>
            </CardHeader>
            <CardContent>
              <div class="flex gap-3 flex-wrap">
                {services.value.map((service) => (
                  <Card
                    key={service.id}
                    onClick={() => {
                      serviceToOpenDialog.value = service;
                    }}
                  >
                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-3">
                      <CardTitle class="text-sm font-medium">
                        {service.name}
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p class="text-xs text-muted-foreground">
                        {service.port}
                      </p>
                    </CardContent>
                    <CardFooter class="flex items-center gap-1">
                      {service.rules?.map((rule) => (
                        <Badge
                          class="text-xs"
                          style={`background-color: ${rule.color};`}
                        >
                          {rule.name}
                        </Badge>
                      ))}
                    </CardFooter>
                  </Card>
                ))}
              </div>
              {serviceToOpenDialog.value && (
                <ServiceDialog
                  data={serviceToOpenDialog.value}
                  ruleList={rules.value}
                  whenSubmitted={handleSubmit}
                  whenClose={() => {
                    serviceToOpenDialog.value = undefined;
                  }}
                ></ServiceDialog>
              )}
            </CardContent>
          </Card>
        </div>
      </div>
    );
  },
});
