//
// Created by davepj007 on 31/08/18.
//

#ifndef ADAPTER_OLDPLAYER_H
#define ADAPTER_OLDPLAYER_H

#include <iostream>
#include <string>

using namespace std;

class OldPlayer { //Esto es lo que se tiene (Reproductor viejo)
public:
    OldPlayer (string song);

    void oldPlay(int volume);

    string song;
};


#endif //ADAPTER_OLDPLAYER_H
