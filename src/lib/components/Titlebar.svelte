<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();

    const minimize = async () => {
        await appWindow.minimize();
    };

    const maximize = async () => {
        await appWindow.toggleMaximize();
    };

    const close = async () => {
        await appWindow.close();
    };
</script>

<div
    class="h-10 bg-black select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0 px-5 py-1.5"
>
    <div data-tauri-drag-region></div>
    <div class="flex">
        <button
            class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-10 bg-transparent hover:bg-amber-400 hover:cursor-pointer"
            id="titlebar-minimize"
            title="minimize"
            onclick={minimize}
        >
            <!-- https://lucide.dev/icons/minus -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="#f1f1f1"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-minus-icon lucide-minus"
            >
                <path d="M5 12h14" />
            </svg>
        </button>
        <button
            class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-10 bg-transparent hover:bg-amber-400 hover:cursor-pointer"
            id="titlebar-maximize"
            title="maximize"
            onclick={maximize}
        >
            <!-- https://lucide.dev/icons/maximize -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="17"
                height="17"
                viewBox="0 0 24 24"
                fill="none"
                stroke="#f1f1f1"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-maximize-icon lucide-maximize"
            >
                <path d="M8 3H5a2 2 0 0 0-2 2v3" />
                <path d="M21 8V5a2 2 0 0 0-2-2h-3" />
                <path d="M3 16v3a2 2 0 0 0 2 2h3" />
                <path d="M16 21h3a2 2 0 0 0 2-2v-3" />
            </svg>
        </button>
        <button
            class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-10 bg-transparent hover:bg-amber-400 hover:cursor-pointer"
            id="titlebar-close"
            title="close"
            onclick={close}
        >
            <!-- https://lucide.dev/icons/x -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="#f1f1f1"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-x-icon lucide-x"
            >
                <path d="M18 6 6 18" />
                <path d="m6 6 12 12" />
            </svg>
        </button>
    </div>
</div>
