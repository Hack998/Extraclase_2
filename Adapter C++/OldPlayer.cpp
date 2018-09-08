//
// Created by davepj007 on 31/08/18.
//

#include "OldPlayer.h"

OldPlayer::OldPlayer (string song) : song(song){} //Constructor

/*
 * Método que se posee, sí recibe el volumen como parametro
 */
void OldPlayer::oldPlay(int volume){
    cout << "\nPlaying song: " << song << " at volume: " << volume << endl;
};