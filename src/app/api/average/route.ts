import axios from 'axios';

import {
  type playlists,
  type PlaylistResponse,
  type PlaylistRequest,
} from '@/spot';

export async function GET(request: Request) {
  const auth = request.headers.get('Authorization');
  let playlistId: string;

  try {
    const body: PlaylistRequest = await request.json();
    playlistId = body.playlistId;
  } catch {
    return new Response(
      'Must provide valid json playlist id. i.e: {"playlistId": "123"}',
      { status: 400 },
    );
  }

  if (auth == null) {
    return new Response('Must provide Spotify authentication token', {
      status: 401,
    });
  }

  const playlist = await axios.get<playlists.Playlist>(
    `https://api.spotify.com/v1/playlists/${playlistId}`,
    {
      headers: {
        Authorization: `Bearer ${auth}`,
      },
    },
  );

  const total = playlist.data.tracks.total;
  let totalDuration = 0;

  for (const track of playlist.data.tracks.items) {
    totalDuration += track.track.duration_ms;
  }

  const averageDuration = totalDuration / total;

  const resp: PlaylistResponse = {
    totalSongs: total,
    totalDuration,
    averageDuration,
  };

  return new Response(JSON.stringify(resp), {
    status: 200,
  });
}
