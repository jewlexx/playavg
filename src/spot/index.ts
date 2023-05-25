import axios from 'axios';

import * as playlists from './playlist';

export { playlists };

export interface PlaylistResponse {
  totalSongs: number;
  totalDuration: number;
  averageDuration: number;
}

export interface PlaylistRequest {
  playlistId: string;
}

export async function downloadUserPlaylists(token: string) {
  const url: string = 'https://api.spotify.com/v1/me/playlists';
  let playlists: playlists.Playlist[] = [];

  const downloadArray = async (url: string) => {
    const resp = await axios.get<playlists.UserPlaylists>(url, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    playlists = playlists.concat(resp.data.items);

    if (resp.data.next) {
      await downloadArray(resp.data.next);
    }
  };

  downloadArray(url);

  return playlists;
}
