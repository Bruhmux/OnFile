export type Category = 'suspect' | 'weapon' | 'location' | 'verdict';

export type Suspect =
  | 'TavernkeepGarrick'
  | 'KnightRowan'
  | 'WizardBjorn'
  | 'ThiefJax'
  | 'PriestThalos'
  | 'AlchemistNox'
  | 'LibrarianMildra'
  | 'MaidAnya';

export type Weapon =
  | 'WoodenStake'
  | 'BloodyDagger'
  | 'PoisonVial'
  | 'RustySword'
  | 'StainedGrimoir'
  | 'SketchyPortal'
  | 'StrangeOoze'
  | 'FlacidWand';

export type Location =
  | 'GuildHall'
  | 'AlchemyLab'
  | 'ThroneRoom'
  | 'TipsyTavern'
  | 'AncientLibrary'
  | 'MysticArcanum'
  | 'RoyalStables'
  | 'MustyCellar';

export const SUSPECTS: { value: Suspect; label: string }[] = [
  { value: 'TavernkeepGarrick', label: 'Tavernkeep Garrick' },
  { value: 'KnightRowan', label: 'Knight Rowan' },
  { value: 'WizardBjorn', label: 'Wizard Bjorn' },
  { value: 'ThiefJax', label: 'Thief Jax' },
  { value: 'PriestThalos', label: 'Priest Thalos' },
  { value: 'AlchemistNox', label: 'Alchemist Nox' },
  { value: 'LibrarianMildra', label: 'Librarian Mildra' },
  { value: 'MaidAnya', label: 'Maid Anya' },
];

export const WEAPONS: { value: Weapon; label: string }[] = [
  { value: 'WoodenStake', label: 'Wooden Stake' },
  { value: 'BloodyDagger', label: 'Bloody Dagger' },
  { value: 'PoisonVial', label: 'Poison Vial' },
  { value: 'RustySword', label: 'Rusty Sword' },
  { value: 'StainedGrimoir', label: 'Stained Grimoir' },
  { value: 'SketchyPortal', label: 'Sketchy Portal' },
  { value: 'StrangeOoze', label: 'Strange Ooze' },
  { value: 'FlacidWand', label: 'Flacid Wand' },
];

export const LOCATIONS: { value: Location; label: string }[] = [
  { value: 'GuildHall', label: 'Guild Hall' },
  { value: 'AlchemyLab', label: 'Alchemy Lab' },
  { value: 'ThroneRoom', label: 'Throne Room' },
  { value: 'TipsyTavern', label: 'Tipsy Tavern' },
  { value: 'AncientLibrary', label: 'Ancient Library' },
  { value: 'MysticArcanum', label: 'Mystic Arcanum' },
  { value: 'RoyalStables', label: 'Royal Stables' },
  { value: 'MustyCellar', label: 'Musty Cellar' },
];

export const CATEGORIES: { value: Category; label: string }[] = [
  { value: 'suspect', label: 'Suspect' },
  { value: 'weapon', label: 'Weapon' },
  { value: 'location', label: 'Location' },
  { value: 'verdict', label: 'Verdict' },
];

export const INDEX_OPTIONS = [0, 1, 2, 3, 4, 5, 6, 7];

export interface Room {
  id: string;
  display_name: string;
  created_at: string;
  is_active: boolean;
  file_data?: unknown;
}

export interface CreateRoomResult {
  room_id: string;
  user_id: string;
  connection_token: string;
}

export interface JoinRoomResult {
  user_id: string;
  connection_token: string;
}

export interface GameCredentials {
  roomId: string;
  userId: string;
  connectionToken: string;
  displayName: string;
}

export interface WsMessage {
  timestamp: string;
  direction: 'sent' | 'received';
  data: unknown;
}

export interface ChatMessage {
  type: 'Chat';
  payload: string;
}

export interface DrawDiscoveryMessage {
  type: 'DrawDiscovery';
}

export interface GuessMessage {
  type: 'Guess';
  payload: {
    suspect: Suspect;
    weapon: Weapon;
    location: Location;
  };
}

export interface PlaceClueMessage {
  type: 'PlaceClue';
  payload: {
    x_category: Category;
    x_idx: number;
    y_category: Category;
    y_idx: number;
    is_true: boolean;
  };
}

export interface ChooseFileMessage {
  type: 'ChooseFile';
  payload: {
    discovery_id: string;
    file_idx: number;
    category: Category;
  };
}

export type DiscoveryCard = 'Wild' | { Same: Category } | { Different: [Category, Category] };

export interface DrawDiscoveryPayload {
  discovery_id: string;
  card: DiscoveryCard;
  files: number;
  total_files: number;
}

export interface InitFilesMessage {
  type: 'InitFiles';
  payload: {
    amount: number;
  };
}

export type ClientMessage =
  | ChatMessage
  | DrawDiscoveryMessage
  | GuessMessage
  | PlaceClueMessage
  | ChooseFileMessage
  | InitFilesMessage;
