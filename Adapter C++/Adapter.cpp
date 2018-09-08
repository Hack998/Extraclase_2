//
// Created by davepj007 on 31/08/18.
//

#include "Adapter.h"
#include "OldPlayer.h"

Adapter::Adapter(string song, int volume) : volume(volume), OldPlayer(song){} //Constructor

void Adapter::play() { //Metodo heredado de IPlayer.
    oldPlay(volume);
};