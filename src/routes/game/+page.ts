import { invoke } from "@tauri-apps/api/tauri";

interface BoardItem {
  value: string;
  marked: boolean;
  uuid: string;
}

type Board = Array<Array<BoardItem>>

export async function load() {
  const letters: string[] = await invoke("init_game");

  const board: Board = await invoke("get_board");

  return {
    letters: letters,
    board: board,
  };
}