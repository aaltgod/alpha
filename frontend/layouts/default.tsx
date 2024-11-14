import { defineComponent } from "vue";
import styles from "./styles.module.css";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
} from "@/components/ui/navigation-menu";
import { NuxtLink } from "#components";
import { Package2 } from "lucide-vue-next";

export default defineComponent({
  setup(_props, { slots }) {
    const projectName = "Alpha";

    const menuItems: { label: string; to: string }[] = [
      {
        label: "Streams",
        to: "/streams",
      },
      {
        label: "Services",
        to: "/services",
      },
    ];

    const $router = useRouter();

    return () => (
      <div class={styles.layout}>
        <header class="top-0 flex h-16 items-center gap-4 border-b bg-background px-4 md:px-6">
          <NavigationMenu class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6">
            <Package2 class="h-6 w-6" />
            <a>{projectName}</a>
            <span class="sr-only">{projectName}</span>
            {menuItems.map((item) => (
              <NavigationMenuItem
                class={
                  $router.currentRoute.value.path == item.to
                    ? "text-foreground transition-colors hover:text-foreground"
                    : "text-muted-foreground transition-colors hover:text-foreground"
                }
              >
                <NuxtLink to={item.to}>
                  <NavigationMenuLink>{item.label}</NavigationMenuLink>
                </NuxtLink>
              </NavigationMenuItem>
            ))}
          </NavigationMenu>
        </header>
        <div class={styles.page}>{slots.default?.()}</div>
      </div>
    );
  },
});
