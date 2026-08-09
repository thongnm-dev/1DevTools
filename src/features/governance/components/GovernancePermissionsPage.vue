<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { useMenuPermissions } from "../composables/useMenuPermissions";
import type { MenuGroup } from "../composables/useMenuPermissions";
import type { UserMenuAccess } from "@/models/menu-permission";

const { t } = useI18n();
const ctrl = useMenuPermissions();

onMounted(() => ctrl.init());

const accessOptions = computed<{ value: UserMenuAccess; label: string; icon: string }[]>(() => [
  { value: "inherit", label: t("governance.permissions.access.inherit"), icon: "pi-link" },
  { value: "allow", label: t("governance.permissions.access.allow"), icon: "pi-check" },
  { value: "deny", label: t("governance.permissions.access.deny"), icon: "pi-ban" },
]);

const subjectLabel = computed(() =>
  ctrl.tab.value === "roles"
    ? (ctrl.selectedRole.value?.name ?? "")
    : (ctrl.selectedUser.value?.username ?? ""),
);

const groupBadgeClass = (group: string) =>
  group === "—"
    ? "badge-neutral"
    : group === "Tools"
      ? "badge-info"
      : "badge-success";

function accessButtonClass(key: string, option: UserMenuAccess) {
  const active = ctrl.accessOf(key) === option;
  if (!active) return "border-divider text-muted hover:bg-canvas";
  if (option === "allow") return "border-brand bg-brand/10 text-brand";
  if (option === "deny") return "border-red-400 bg-red-50 text-red-600 dark:bg-red-500/15 dark:text-red-300";
  return "border-divider bg-canvas text-secondary";
}

function groupCheckboxIcon(group: MenuGroup) {
  const state = ctrl.groupState(group);
  if (state === "all") return "pi-check-square";
  if (state === "some") return "pi-minus-circle";
  return "pi-stop";
}
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">
    <!-- Tabs + actions -->
    <section class="flex flex-wrap items-center gap-3 rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <div class="flex rounded-md border border-divider p-0.5">
        <Button
          v-for="tabKey in (['roles', 'users'] as const)"
          :key="tabKey"
          :class="[
            'rounded px-4 py-1.5 text-sm font-bold capitalize transition',
            ctrl.tab.value === tabKey ? 'bg-brand text-white' : 'text-muted hover:bg-canvas',
          ]"
          unstyled
          @click="ctrl.switchTab(tabKey)"
        >
          <i :class="['pi mr-1.5 text-xs', tabKey === 'roles' ? 'pi-shield' : 'pi-user']" />
          {{ tabKey === "roles" ? t("governance.permissions.tabs.roles") : t("governance.permissions.tabs.users") }}
        </Button>
      </div>

      <label class="block min-w-0 flex-1">
        <span class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3 focus-within:border-brand focus-within:ring-2 focus-within:ring-brand/20">
          <i class="pi pi-search shrink-0 text-muted" />
          <InputText
            class="embedded-input min-w-0 flex-1 border-0 bg-transparent p-0 text-sm text-ink outline-none shadow-none"
            :placeholder="t('governance.permissions.form.searchPlaceholder')"
            :model-value="ctrl.searchQuery.value"
            @update:model-value="ctrl.searchQuery.value = ($event as string) ?? ''"
          />
        </span>
      </label>

      <Button
        :label="t('governance.permissions.actions.revert')"
        icon="pi pi-undo"
        severity="secondary"
        outlined
        size="small"
        :disabled="!ctrl.dirty.value || ctrl.saving.value"
        :title="t('governance.permissions.actions.revertTitle')"
        @click="ctrl.revert()"
      />
      <Button
        :label="t('governance.permissions.actions.save')"
        icon="pi pi-save"
        size="small"
        :loading="ctrl.saving.value"
        :disabled="!ctrl.hasSelection.value || !ctrl.dirty.value"
        @click="ctrl.save()"
      />
    </section>

    <!-- Feedback -->
    <p v-if="ctrl.error.value" class="banner-danger">
      {{ ctrl.error.value }}
    </p>
    <p v-else-if="ctrl.savedMessage.value" class="banner-success">
      {{ ctrl.savedMessage.value }}
    </p>

    <div class="grid min-h-0 flex-1 gap-4 lg:grid-cols-[260px_minmax(0,1fr)]">
      <!-- Subject list -->
      <section class="min-h-0 overflow-auto rounded-lg border border-divider bg-panel shadow-sm">
        <p class="sticky top-0 z-10 border-b border-divider bg-panel px-4 py-3 text-xs font-bold text-ink">
          {{ ctrl.tab.value === "roles" ? t("governance.permissions.list.rolesHeader") : t("governance.permissions.list.usersHeader") }}
        </p>

        <template v-if="ctrl.tab.value === 'roles'">
          <Button
            v-for="role in ctrl.roles.value"
            :key="role.id"
            :class="[
              'block w-full border-b border-divider px-4 py-2.5 text-left transition',
              ctrl.selectedRoleId.value === role.id
                ? 'border-l-2 border-l-brand bg-brand/15'
                : 'hover:bg-canvas',
            ]"
            unstyled
            @click="ctrl.selectRole(role.id)"
          >
            <span class="block text-sm font-semibold text-ink">{{ role.name }}</span>
            <span class="block text-xs text-muted">{{ t("governance.permissions.list.userCount", { count: role.user_count }) }}</span>
          </Button>
          <p v-if="ctrl.roles.value.length === 0" class="p-6 text-center text-sm text-muted">
            {{ t("governance.permissions.list.noRoles") }}
          </p>
        </template>

        <template v-else>
          <Button
            v-for="user in ctrl.users.value"
            :key="user.id"
            :class="[
              'block w-full border-b border-divider px-4 py-2.5 text-left transition',
              ctrl.selectedUserId.value === user.id
                ? 'border-l-2 border-l-brand bg-brand/15'
                : 'hover:bg-canvas',
            ]"
            unstyled
            @click="ctrl.selectUser(user.id)"
          >
            <span class="block text-sm font-semibold text-ink">{{ user.username }}</span>
            <span class="block truncate text-xs text-muted">
              {{ user.roles.join(", ") || t("governance.permissions.list.noRole") }}
            </span>
          </Button>
          <p v-if="ctrl.users.value.length === 0" class="p-6 text-center text-sm text-muted">
            {{ t("governance.permissions.list.noUsers") }}
          </p>
        </template>
      </section>

      <!-- Menu permission matrix -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <header class="flex flex-wrap items-center gap-3 border-b border-divider px-4 py-3">
          <div class="min-w-0 flex-1">
            <h3 class="truncate section-title">
              {{ subjectLabel || (ctrl.tab.value === "roles" ? t("governance.permissions.matrix.selectRole") : t("governance.permissions.matrix.selectUser")) }}
            </h3>
            <p class="text-xs text-muted">
              {{ t("governance.permissions.matrix.allowedSummary", { granted: ctrl.grantedCount.value, total: ctrl.menus.value.length }) }}
              <span v-if="ctrl.dirty.value" class="ml-1 font-bold text-amber-600">{{ t("governance.permissions.matrix.unsaved") }}</span>
            </p>
          </div>

          <div v-if="ctrl.hasSelection.value" class="flex items-center gap-1">
            <Button :label="t('governance.permissions.actions.allowAll')" size="small" text @click="ctrl.selectAll()" />
            <Button
              :label="ctrl.tab.value === 'roles' ? t('governance.permissions.actions.clearAll') : t('governance.permissions.actions.denyAll')"
              size="small"
              text
              severity="secondary"
              @click="ctrl.clearAll()"
            />
            <Button
              v-if="ctrl.tab.value === 'users'"
              :label="t('governance.permissions.actions.resetToInherit')"
              size="small"
              text
              severity="secondary"
              @click="ctrl.resetToInherit()"
            />
          </div>
        </header>

        <div v-if="!ctrl.hasSelection.value" class="flex flex-1 items-center justify-center p-6 text-sm text-muted">
          {{ ctrl.tab.value === "roles" ? t("governance.permissions.matrix.emptyPromptRole") : t("governance.permissions.matrix.emptyPromptUser") }}
        </div>

        <div v-else class="min-h-0 flex-1 overflow-auto">
          <div v-for="group in ctrl.menuGroups.value" :key="group.name" class="border-b border-divider last:border-b-0">
            <!-- Group header -->
            <div class="flex items-center gap-2 bg-canvas px-4 py-2">
              <Button
                v-if="ctrl.tab.value === 'roles'"
                class="flex items-center gap-2 text-left"
                :title="ctrl.groupState(group) === 'all' ? t('governance.permissions.matrix.clearGroup') : t('governance.permissions.matrix.allowGroup')"
                unstyled
                @click="ctrl.toggleGroup(group, ctrl.groupState(group) !== 'all')"
              >
                <i :class="['pi text-sm', groupCheckboxIcon(group), ctrl.groupState(group) === 'none' ? 'text-muted' : 'text-brand']" />
                <span :class="groupBadgeClass(group.name)">
                  {{ group.name }}
                </span>
              </Button>
              <span
                v-else
                :class="groupBadgeClass(group.name)"
              >
                {{ group.name }}
              </span>
              <span class="text-xs text-muted">{{ t("governance.permissions.matrix.menuCount", { count: group.items.length }) }}</span>
            </div>

            <!-- Menu rows -->
            <div
              v-for="item in group.items"
              :key="item.key"
              class="flex items-center gap-3 border-t border-divider px-4 py-2.5"
            >
              <i :class="`pi ${item.icon} shrink-0 text-muted`" />
              <div class="min-w-0 flex-1">
                <p class="truncate text-sm font-semibold text-ink">
                  {{ item.title }}
                  <span v-if="!item.visible" class="ml-1 text-[11px] font-normal text-muted">{{ t("governance.permissions.matrix.hidden") }}</span>
                </p>
                <p class="truncate font-mono text-xs text-secondary">{{ item.path }}</p>
              </div>

              <!-- Roles tab: single grant checkbox -->
              <Button
                v-if="ctrl.tab.value === 'roles'"
                :icon="ctrl.isGranted(item.key) ? 'pi pi-check-square' : 'pi pi-stop'"
                :label="ctrl.isGranted(item.key) ? t('governance.permissions.matrix.allowed') : t('governance.permissions.matrix.notAllowed')"
                size="small"
                :outlined="!ctrl.isGranted(item.key)"
                :severity="ctrl.isGranted(item.key) ? undefined : 'secondary'"
                :class="['w-36 shrink-0', ctrl.isGranted(item.key) ? 'border-brand bg-brand/10 text-brand' : '']"
                @click="ctrl.toggleRoleGrant(item.key)"
              />

              <!-- Users tab: inherit / allow / deny -->
              <div v-else class="flex shrink-0 items-center gap-3">
                <span class="w-28 text-right text-xs text-muted">
                  {{ t("governance.permissions.matrix.fromRole") }}
                  <span :class="ctrl.inheritedAllowed(item.key) ? 'font-bold text-brand' : 'font-bold text-muted'">
                    {{ ctrl.inheritedAllowed(item.key) ? t("governance.permissions.matrix.inheritedAllowed") : t("governance.permissions.matrix.inheritedDenied") }}
                  </span>
                </span>
                <div class="flex rounded-md border border-divider p-0.5">
                  <Button
                    v-for="opt in accessOptions"
                    :key="opt.value"
                    :class="[
                      'flex items-center gap-1 rounded border px-2.5 py-1 text-xs font-bold transition',
                      accessButtonClass(item.key, opt.value),
                    ]"
                    :title="
                      opt.value === 'inherit'
                        ? t('governance.permissions.access.inheritTitle')
                        : opt.value === 'allow'
                          ? t('governance.permissions.access.allowTitle')
                          : t('governance.permissions.access.denyTitle')
                    "
                    unstyled
                    @click="ctrl.setUserAccess(item.key, opt.value)"
                  >
                    <i :class="`pi ${opt.icon} text-[10px]`" />
                    {{ opt.label }}
                  </Button>
                </div>
              </div>
            </div>
          </div>

          <p v-if="ctrl.menuGroups.value.length === 0" class="p-6 text-center text-sm text-muted">
            {{ t("governance.permissions.matrix.empty") }}
          </p>
        </div>
      </section>
    </div>
  </section>
</template>
